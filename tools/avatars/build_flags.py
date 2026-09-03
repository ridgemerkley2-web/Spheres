#!/usr/bin/env python3
"""Build the selector's single embedded SVG flag sheet.

Ordinary flags come from flag-icons 7.5.0 (MIT). Historical state/date
variants are fetched from their exact Wikimedia Commons file pages and cached
beside this script with source and license metadata.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import re
import time
import urllib.error
import urllib.parse
import urllib.request
from datetime import date
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "spheres-web" / "data" / "nation_figures.json"
OUTPUT = ROOT / "spheres-web" / "ui" / "nation-flags-v1.svg"
CACHE = Path(__file__).resolve().parent / "historical_flags"
USER_AGENT = "SPHERES-game-flag-builder/1.0 (local development; Wikimedia Commons client)"
FLAG_ICONS_VERSION = "7.5.0"
REQUEST_INTERVAL_SECONDS = 1.0
RATE_LIMIT_BACKOFF_SECONDS = (2.0, 5.0, 10.0, 20.0)
_last_request_at = 0.0

# Names the current flag-icons catalog does not spell the way the simulation
# does. Defunct states still need an ISO fallback here so a failed network
# refresh never leaves the sheet without a symbol.
CODE_OVERRIDES = {
    "USA": "us",
    "USSR": None,
    "Turkey": "tr",
    "Yugoslavia": None,
    "Bosnia": "ba",
    "Czechoslovakia": "cz",
    "Zaire": "cd",
    "Congo": "cg",
    "CapeVerde": "cv",
    "Samoa": "ws",
    "Brunei": "bn",
    "EastTimor": "tl",
    "Macedonia": "mk",
    "Swaziland": "sz",
}

# The date is part of the file title on purpose: this is a January 1990 game,
# and modern ISO art is wrong for these seated states. Successor flags use the
# first broadly adopted independence-era design when it differs materially.
HISTORICAL_TITLES = {
    "USSR": "Flag of the Soviet Union.svg",
    "Russia": "Flag of Russia (1991–1993).svg",
    "Yugoslavia": "Flag of Yugoslavia (1946–1992).svg",
    "Zaire": "Flag of Zaire (1971–1997).svg",
    "Afghanistan": "Flag of Afghanistan (1987–1992).svg",
    "Albania": "Flag of Albania (1970s–1980s).svg",
    "Bulgaria": "Flag of Bulgaria (1971–1990).svg",
    "Cambodia": "Flag of the State of Cambodia (1989–1992).svg",
    "Congo": "Flag of the People's Republic of the Congo.svg",
    "Ethiopia": "Flag of Ethiopia (1987–1991).svg",
    "Iraq": "Flag of Iraq (1963–1991).svg",
    "Libya": "Flag of Libya (1977–2011).svg",
    "Mongolia": "Flag of the Mongolian People's Republic (1945–1992).svg",
    "Myanmar": "Flag of Myanmar (1974–2010).svg",
    "SouthAfrica": "Flag of South Africa (1928–1994, dark colors).svg",
    "Syria": "Flag of the United Arab Republic (1958–1971), Flag of Syria (1980–2024).svg",
    "Lebanon": "Flag of Lebanon (1943-1990).svg",
    "Bahrain": "Flag of Bahrain (1972–2002).svg",
    "Brazil": "Flag of Brazil (1968–1992).svg",
    "CapeVerde": "Flag of Cape Verde (1975–1992).svg",
    "Comoros": "Flag of the Comoros (1978–1992).svg",
    "Lesotho": "Flag of Lesotho (1987–2006).svg",
    "Oman": "Flag of Oman (1970–1995).svg",
    "Seychelles": "Flag of Seychelles (1977–1996).svg",
    "Venezuela": "Flag of Venezuela (1930–2006).svg",
    "Belarus": "Flag of Belarus (1918, 1991–1995).svg",
    "Bosnia": "Flag of Bosnia and Herzegovina (1992–1998).svg",
    "Georgia": "Flag of Georgia (1990–2004).svg",
    "Honduras": "Flag of Honduras (1949–2022, 2026–present).svg",
    "Kyrgyzstan": "Flag of Kyrgyzstan (1992–2023).svg",
    "Macedonia": "Flag of Macedonia (1992–1995).svg",
    "Montenegro": "Flag of Montenegro (1993–2004).svg",
    "Serbia": "Flag of Serbia (1992–2004).svg",
    "Turkmenistan": "Flag of Turkmenistan (1992–1997).svg",
    "Zambia": "Flag of Zambia (1964–1996).svg",
}


def request_bytes(url: str, timeout: int) -> bytes:
    """Fetch one Commons resource, pacing requests and backing off on HTTP 429."""
    global _last_request_at

    for attempt in range(len(RATE_LIMIT_BACKOFF_SECONDS) + 1):
        elapsed = time.monotonic() - _last_request_at
        if elapsed < REQUEST_INTERVAL_SECONDS:
            time.sleep(REQUEST_INTERVAL_SECONDS - elapsed)

        request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
        _last_request_at = time.monotonic()
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                return response.read()
        except urllib.error.HTTPError as error:
            if error.code != 429 or attempt == len(RATE_LIMIT_BACKOFF_SECONDS):
                raise
            retry_after = error.headers.get("Retry-After", "")
            try:
                delay = max(float(retry_after), RATE_LIMIT_BACKOFF_SECONDS[attempt])
            except (TypeError, ValueError):
                delay = RATE_LIMIT_BACKOFF_SECONDS[attempt]
            # Do not let an unexpectedly large server hint stall a local build forever.
            delay = min(delay, 60.0)
            print(f"rate limited by Commons; retrying in {delay:g}s")
            time.sleep(delay)

    raise AssertionError("unreachable")


def request_json(url: str, params: dict[str, str]) -> dict:
    raw = request_bytes(url + "?" + urllib.parse.urlencode(params), timeout=45)
    return json.loads(raw.decode("utf-8"))


def strip_html(value: str) -> str:
    return re.sub(r"\s+", " ", html.unescape(re.sub(r"<[^>]+>", " ", value))).strip()


def fetch_historical(nation_id: str, title: str, refresh: bool) -> tuple[str, dict, bool]:
    target = CACHE / f"{nation_id}.svg"
    source_file = CACHE / "sources.json"
    sources = json.loads(source_file.read_text(encoding="utf-8")) if source_file.exists() else {}
    if target.exists() and nation_id in sources and not refresh:
        return target.read_text(encoding="utf-8"), sources, False

    payload = request_json(
        "https://commons.wikimedia.org/w/api.php",
        {
            "action": "query",
            "format": "json",
            "titles": "File:" + title,
            "prop": "imageinfo",
            "iiprop": "url|extmetadata",
            "redirects": "1",
        },
    )
    page = next(iter(payload.get("query", {}).get("pages", {}).values()), {})
    if "missing" in page or not page.get("imageinfo"):
        raise RuntimeError(f"Commons has no exact file named {title}")
    info = page["imageinfo"][0]
    metadata = info.get("extmetadata", {})
    license_name = strip_html(metadata.get("LicenseShortName", {}).get("value", ""))
    if not any(mark in license_name.lower() for mark in ("public domain", "cc0", "cc by", "cc-by")):
        raise RuntimeError(f"{title} has no accepted free license ({license_name or 'none'})")
    raw = request_bytes(info["url"], timeout=60)
    svg = raw.decode("utf-8-sig")
    if "<svg" not in svg:
        raise RuntimeError(f"{title} did not download as SVG")

    CACHE.mkdir(parents=True, exist_ok=True)
    target.write_text(svg, encoding="utf-8", newline="\n")
    sources[nation_id] = {
        "file_title": page.get("title", "File:" + title),
        "source_url": info.get("descriptionurl", ""),
        "original_url": info.get("url", ""),
        "license": license_name,
        "license_url": strip_html(metadata.get("LicenseUrl", {}).get("value", "")),
        "creator": strip_html(metadata.get("Artist", {}).get("value", "")),
        "sha256": hashlib.sha256(raw).hexdigest(),
        "retrieved": date.today().isoformat(),
    }
    source_file.write_text(json.dumps(sources, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return svg, sources, True


def prefix_ids(inner: str, nation_id: str) -> str:
    """Namespace SVG IDs without mistaking CSS colors for fragment links."""
    ids = set(re.findall(r'\bid\s*=\s*["\']([^"\']+)["\']', inner))
    replacements = {old: f"flag-{nation_id}-{old}" for old in ids}

    def replace_id(match: re.Match[str]) -> str:
        return match.group(1) + replacements[match.group(2)] + match.group(3)

    def replace_fragment(match: re.Match[str]) -> str:
        target = replacements.get(match.group(2), match.group(2))
        return match.group(1) + "#" + target + match.group(3)

    def replace_url(match: re.Match[str]) -> str:
        quote, old = match.group(1), match.group(2)
        target = replacements.get(old, old)
        return f"url({quote}#{target}{quote})"

    inner = re.sub(
        r'(\bid\s*=\s*["\'])([^"\']+)(["\'])',
        replace_id,
        inner,
    )
    inner = re.sub(
        r'(\b(?:href|xlink:href)\s*=\s*["\'])#([^"\']+)(["\'])',
        replace_fragment,
        inner,
    )
    inner = re.sub(
        r'url\(\s*(["\']?)#([A-Za-z_][\w:.-]*)\1\s*\)',
        replace_url,
        inner,
    )
    return inner


def as_symbol(svg: str, nation_id: str) -> str:
    root = re.search(r"<svg\b([^>]*)>(.*)</svg>\s*$", svg, re.DOTALL | re.IGNORECASE)
    if not root:
        raise RuntimeError(f"{nation_id} asset has no parseable SVG root")
    attrs, inner = root.groups()
    viewbox_match = re.search(r'viewBox=["\']([^"\']+)["\']', attrs, re.IGNORECASE)
    if viewbox_match:
        viewbox = viewbox_match.group(1)
    else:
        width = re.search(r'width=["\']([0-9.]+)', attrs, re.IGNORECASE)
        height = re.search(r'height=["\']([0-9.]+)', attrs, re.IGNORECASE)
        viewbox = f"0 0 {width.group(1) if width else 640} {height.group(1) if height else 480}"
    # Some Commons SVGs retain editor-specific elements/attributes in their
    # body. Keep the namespace declarations that made those nodes valid on
    # the source root when moving the body under a standalone <symbol>.
    namespace_declarations = re.findall(
        r'\bxmlns:[A-Za-z_][\w.-]*\s*=\s*(?:"[^"]*"|\'[^\']*\')', attrs
    )
    namespace_suffix = " " + " ".join(namespace_declarations) if namespace_declarations else ""
    # Editing-tool metadata is not part of the flag and often depends on
    # namespace declarations carried only by the source root. Dropping it
    # keeps the combined sheet small and prevents unbound rdf/cc/dc prefixes.
    inner = re.sub(r"<metadata\b.*?</metadata>", "", inner, flags=re.DOTALL | re.IGNORECASE)
    inner = prefix_ids(inner, nation_id)
    return (
        f'  <symbol id="flag-{nation_id}" viewBox="{viewbox}"{namespace_suffix}>'
        f"\n{inner.strip()}\n  </symbol>"
    )


def neutral_symbol(nation_id: str) -> str:
    return (
        f'  <symbol id="flag-{nation_id}" viewBox="0 0 640 480">'
        '<path fill="#c9bfd5" d="M0 0h640v480H0z"/>'
        '<path fill="#99c8ca" d="M0 320h640v160H0z"/>'
        f'<text x="320" y="276" text-anchor="middle" fill="#fff" font-size="86" '
        f'font-family="sans-serif" font-weight="700">{nation_id[:3].upper()}</text></symbol>'
    )


def load_flag_icons_catalog(path: Path, parser: argparse.ArgumentParser) -> list[dict]:
    catalog_file = path / "country.json"
    package_file = path / "package.json"
    flag_directory = path / "flags" / "4x3"
    if not catalog_file.is_file() or not package_file.is_file() or not flag_directory.is_dir():
        parser.error(
            f"flag-icons {FLAG_ICONS_VERSION} is missing or incomplete at {path}; "
            "expected package.json, country.json, and flags/4x3. "
            "See tools/avatars/README.md#build-the-flag-sprite."
        )

    try:
        package = json.loads(package_file.read_text(encoding="utf-8"))
        catalog = json.loads(catalog_file.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        parser.error(f"could not read flag-icons catalog at {path}: {error}")
    if package.get("name") != "flag-icons" or package.get("version") != FLAG_ICONS_VERSION:
        parser.error(
            f"{path} is flag-icons {package.get('version', 'unknown')}; "
            f"this generator requires exactly {FLAG_ICONS_VERSION}."
        )
    if not isinstance(catalog, list):
        parser.error(f"{catalog_file} must contain the flag-icons country list")
    return catalog


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--flag-icons",
        type=Path,
        default=ROOT / ".tmp-flag-icons" / "package",
        help="extracted flag-icons 7.5.0 package directory",
    )
    parser.add_argument("--refresh-historical", action="store_true")
    parser.add_argument("--offline", action="store_true", help="use only already cached historical files")
    args = parser.parse_args()

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    nations = manifest.get("nations", {})
    catalog = load_flag_icons_catalog(args.flag_icons, parser)
    codes_by_name = {entry["name"].lower(): entry["code"] for entry in catalog}
    symbols: list[str] = []
    warnings: list[str] = []
    historical_fetched = 0
    historical_cached = 0
    historical_fallbacks = 0
    neutral_fallbacks = 0

    for nation_id, nation in nations.items():
        display = nation["display_name"]
        code = CODE_OVERRIDES.get(nation_id, codes_by_name.get(display.lower()))
        svg = None
        historical_title = HISTORICAL_TITLES.get(nation_id)
        if historical_title:
            try:
                if args.offline and not (CACHE / f"{nation_id}.svg").exists():
                    raise RuntimeError("historical variant is not cached")
                svg, _, fetched = fetch_historical(
                    nation_id, historical_title, args.refresh_historical
                )
                if fetched:
                    historical_fetched += 1
                else:
                    historical_cached += 1
            except Exception as error:
                historical_fallbacks += 1
                warnings.append(f"{nation_id}: {error}; using current/neutral fallback")
        if svg is None and code:
            source = args.flag_icons / "flags" / "4x3" / f"{code}.svg"
            if source.exists():
                svg = source.read_text(encoding="utf-8")
        if svg:
            symbols.append(as_symbol(svg, nation_id))
        else:
            neutral_fallbacks += 1
            symbols.append(neutral_symbol(nation_id))

    header = (
        '<svg xmlns="http://www.w3.org/2000/svg" '
        'xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true">\n'
        '  <!-- Generated by tools/avatars/build_flags.py. Ordinary flags: flag-icons 7.5.0 (MIT). -->\n'
    )
    OUTPUT.write_text(header + "\n".join(symbols) + "\n</svg>\n", encoding="utf-8", newline="\n")
    print(f"wrote {len(symbols)} flag symbols to {OUTPUT.relative_to(ROOT)}")
    print(
        "historical assets: "
        f"{historical_fetched} fetched, {historical_cached} cached, "
        f"{historical_fallbacks} fell back ({neutral_fallbacks} neutral symbols)"
    )
    for warning in warnings:
        print("warning: " + warning)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
