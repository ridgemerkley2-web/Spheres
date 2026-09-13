"""One disposable process, one saved world, bounded stdout.

No native campaign mutation. Canonical output is exclusive-create. The caller
compares canonical files byte-for-byte and retains the unchanged raw archives.
Numbers use IEEE754 binary64, matching the previous JS JSON/deepEqual semantics,
including negative zero. Only --ignore-maintenance-plan changes comparison input.
"""

import argparse
import hashlib
import json
import math
import pathlib
import struct
import sys

FORMAT = "s08-typed-canonical-world-v1-js-f64"
MAX_IDS = 100_000
MAX_MATCHES = 128
MAX_STDOUT_BYTES = 4 * 1024 * 1024
MAX_CANONICAL_BYTES = 4 * 1024 * 1024 * 1024
SENTINEL = "S08:explicitly-ignored-buyer-maintenance-plan"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def file_hash(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def parse_integer(text):
    # JSON.parse("-0") is negative zero; Python's int("-0") is not.
    return -0.0 if text == "-0" else int(text)


def reject_constant(text):
    raise ValueError("Non-JSON numeric constant: " + text)


class Sink:
    """Batch small typed values; never stringify or buffer the whole world."""

    def __init__(self, output=None):
        self.output = output
        self.digest = hashlib.sha256()
        self.bytes = 0
        self.buffer = bytearray()

    def write(self, value):
        self.bytes += len(value)
        require(self.bytes <= MAX_CANONICAL_BYTES, "Canonical stream exceeds its declared 4 GiB bound")
        if len(value) >= 65536:
            self.flush()
            self.digest.update(value)
            if self.output is not None:
                self.output.write(value)
        else:
            self.buffer.extend(value)
            if len(self.buffer) >= 65536:
                self.flush()

    def flush(self):
        if self.buffer:
            self.digest.update(self.buffer)
            if self.output is not None:
                self.output.write(self.buffer)
            self.buffer.clear()


def canonical(value, sink, ignored_equipment=None):
    if value is None:
        sink.write(b"n")
    elif value is False:
        sink.write(b"f")
    elif value is True:
        sink.write(b"t")
    elif isinstance(value, (int, float)):
        number = float(value)
        require(math.isfinite(number), "Nonfinite JSON number in native world")
        sink.write(b"d" + struct.pack(">d", number))
    elif isinstance(value, str):
        encoded = value.encode("utf-8", errors="surrogatepass")
        sink.write(b"s" + struct.pack(">Q", len(encoded)))
        sink.write(encoded)
    elif isinstance(value, list):
        sink.write(b"a" + struct.pack(">Q", len(value)))
        for child in value:
            canonical(child, sink, ignored_equipment)
    elif isinstance(value, dict):
        require(all(isinstance(key, str) for key in value), "Non-string JSON object key")
        # Normalize only this single permitted field, including absent -> added.
        # Shallow key/value iteration never copies the native world.
        ignored = value is ignored_equipment
        keys = sorted(set(value) | ({"maintenance_plan"} if ignored else set()))
        sink.write(b"o" + struct.pack(">Q", len(keys)))
        for key in keys:
            canonical(key, sink)
            child = SENTINEL if ignored and key == "maintenance_plan" else value[key]
            canonical(child, sink, ignored_equipment)
    else:
        raise ValueError("Unexpected non-JSON value type: " + type(value).__name__)


def value_hash(value):
    sink = Sink()
    canonical(value, sink)
    sink.flush()
    return sink.digest.hexdigest()


def unique(rows, label, optional=False):
    require(len(rows) <= 1, "Duplicate " + label)
    require(optional or len(rows) == 1, "Missing " + label)
    return rows[0] if rows else None


def bounded(rows, limit, label):
    require(len(rows) <= limit, label + " exceeds the declared projection bound; nothing was truncated")
    return rows


def compare_files(left_path, right_path):
    """Streaming first difference: --compare LEFT_CANONICAL RIGHT_CANONICAL.

    Token grammar: n/f/t; d + big-endian binary64; s + big-endian u64 UTF-8
    length + bytes; a + u64 count + values; o + u64 count + (string key,value).
    Only two buffered files and at most 64 KiB of value bytes are retained.
    """
    with pathlib.Path(left_path).open("rb") as left, pathlib.Path(right_path).open("rb") as right:
        def read(stream, count):
            value = stream.read(count)
            require(len(value) == count, "Truncated canonical comparison input")
            return value

        def difference(path, kind, a, b, offsets):
            return {"equal": False, "path": list(path), "kind": kind,
                    "left": a, "right": b, "token_offsets": offsets}

        def walk(path):
            offsets = {"left": left.tell(), "right": right.tell()}
            a, b = read(left, 1), read(right, 1)
            if a != b:
                return difference(path, "type", a.hex(), b.hex(), offsets)
            if a in (b"n", b"f", b"t"):
                return None
            if a == b"d":
                av, bv = read(left, 8), read(right, 8)
                if av != bv:
                    return difference(path, "number-binary64", av.hex(), bv.hex(), offsets)
                return None
            require(a in (b"s", b"a", b"o"), "Unknown canonical token")
            ac, bc = struct.unpack(">Q", read(left, 8))[0], struct.unpack(">Q", read(right, 8))[0]
            if ac != bc:
                return difference(path, "length-or-count", ac, bc, offsets)
            if a == b"s":
                remaining = ac
                while remaining:
                    size = min(remaining, 65536)
                    av, bv = read(left, size), read(right, size)
                    if av != bv:
                        at = next(i for i, values in enumerate(zip(av, bv)) if values[0] != values[1])
                        result = difference(path, "string-bytes", av[max(0, at-16):at+16].hex(),
                                            bv[max(0, at-16):at+16].hex(), offsets)
                        result["string_byte_offset"] = ac - remaining + at
                        return result
                    remaining -= size
                return None
            if a == b"a":
                for index in range(ac):
                    result = walk(path + (index,))
                    if result:
                        return result
                return None
            for _ in range(ac):
                key_offsets = {"left": left.tell(), "right": right.tell()}
                require(read(left, 1) == b"s" and read(right, 1) == b"s", "Object key must be a string token")
                al = struct.unpack(">Q", read(left, 8))[0]
                bl = struct.unpack(">Q", read(right, 8))[0]
                require(al <= 65536 and bl <= 65536, "Canonical key exceeds the diagnostic 64 KiB bound")
                ak, bk = read(left, al), read(right, bl)
                if ak != bk:
                    return difference(path, "object-key", ak[:128].hex(), bk[:128].hex(), key_offsets)
                result = walk(path + (ak.decode("utf-8", errors="surrogatepass"),))
                if result:
                    return result
            return None

        result = walk(())
        if result is None:
            require(not left.read(1) and not right.read(1), "Trailing canonical comparison data")
            result = {"equal": True, "bytes_compared": left.tell()}
        sys.stdout.write(json.dumps(result, ensure_ascii=True, allow_nan=False) + "\n")


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--compare":
        require(len(sys.argv) == 4, "Use --compare LEFT_CANONICAL RIGHT_CANONICAL")
        compare_files(sys.argv[2], sys.argv[3])
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input")
    parser.add_argument("output_canonical")
    parser.add_argument("buyer")
    parser.add_argument("--ignore-maintenance-plan", action="store_true")
    parser.add_argument("--revision")
    parser.add_argument("--seller")
    parser.add_argument("--company", type=int)
    parser.add_argument("--product", type=int)
    args = parser.parse_args()
    source = pathlib.Path(args.input).resolve(strict=True)
    output = pathlib.Path(args.output_canonical).resolve()
    require(source != output, "Canonical output must differ from the input archive")
    require(not output.exists(), "Refuse to overwrite an existing canonical output")
    require(output.parent.is_dir(), "Canonical output parent must already exist")
    before_hash = file_hash(source)
    before_bytes = source.stat().st_size
    with source.open("r", encoding="utf-8-sig") as stream:
        world = json.load(stream, parse_int=parse_integer, parse_constant=reject_constant)
    while isinstance(world, dict) and isinstance(world.get("world"), dict):
        world = world["world"]
    require(isinstance(world, dict) and isinstance(world.get("nations"), list), "Archive must contain a native world with nations")
    indexed = [(index, nation) for index, nation in enumerate(world["nations"])
               if nation.get("id") == args.buyer]
    index, buyer = unique(indexed, "buyer nation " + args.buyer)
    equipment = buyer.get("equipment")
    require(equipment is None or isinstance(equipment, dict), "Malformed buyer equipment book")
    if args.ignore_maintenance_plan:
        require(isinstance(equipment, dict), "Ignoring maintenance requires an existing buyer equipment book")
    eq = equipment or {}
    revisions = eq.get("revisions") or {}
    require(isinstance(revisions, dict), "Malformed buyer revision book")
    revision_ids = bounded(sorted(revisions), MAX_IDS, "Buyer revision IDs")
    learned = eq.get("learned")
    if learned is None:
        learned = []
    require(isinstance(learned, list), "Malformed buyer component-research list")
    holdings = buyer.get("arsenal", {}).get("held", [])
    require(isinstance(holdings, list), "Malformed buyer holdings")
    held_by_revision = {}
    for holding in holdings:
        revision = holding.get("design_id")
        amount = holding.get("units")
        require(revision is None or isinstance(revision, str), "Malformed held revision ID")
        require(isinstance(amount, (int, float)) and not isinstance(amount, bool)
                and math.isfinite(float(amount)) and amount >= 0, "Malformed held unit amount")
        # Legacy catalogue holdings have no custom revision ID. Their full
        # objects remain in the canonical world; only this exact-ID projection
        # skips them, matching the runner's original design_id === revision.
        if revision is None:
            continue
        held_by_revision[revision] = held_by_revision.get(revision, 0) + amount
    bounded(held_by_revision, MAX_IDS, "Held revision totals")

    firms = world.get("companies", {}).get("firms", [])
    contracts = world.get("companies", {}).get("imports", {}).get("contracts", [])
    require(isinstance(firms, list) and isinstance(contracts, list), "Malformed company/import lists")
    ids = bounded([row["id"] for row in contracts], MAX_IDS, "Import contract IDs")
    matches = bounded([row for row in contracts if row.get("buyer") == args.buyer
                       and (args.seller is None or row.get("seller") == args.seller)
                       and (args.company is None or row.get("company") == args.company)
                       and (args.product is None or row.get("product") == args.product)],
                      MAX_MATCHES, "Matching import contracts")
    supplier = None
    if args.company is not None:
        firm = unique([firm for firm in firms if firm.get("id") == args.company
                       and (args.seller is None or firm.get("nation") == args.seller)],
                      "requested supplier", optional=True)
        if firm is not None:
            product = None
            if args.product is not None:
                product = unique([product for product in firm.get("products", [])
                                  if product.get("id") == args.product],
                                 "requested supplier equipment product", optional=True)
            supplier = {"id": firm["id"], "nation": firm["nation"],
                        "product": None if product is None else
                        {"id": product["id"], "stock": product["stock"]}}
    program = buyer.get("program_budget")
    program_fields = None if program is None else {
        key: program.get(key) for key in
        ("day", "settled_day", "spent_today_bn", "prepaid_used_today_bn")}
    facts = {
        "version": 1,
        "input": {"path": str(source), "bytes": before_bytes, "sha256": before_hash},
        "requested": {"buyer": args.buyer, "revision": args.revision,
                      "seller": args.seller, "company": args.company, "product": args.product},
        "buyer": {"id": args.buyer, "tech_sha256": value_hash(buyer.get("tech")),
                  "learned_sha256": value_hash(learned), "revision_ids": revision_ids,
                  "held_by_revision": held_by_revision,
                  "held_units": held_by_revision.get(args.revision, 0),
                  "buyer_revision": revisions.get(args.revision),
                  "maintenance_plan": eq.get("maintenance_plan"), "program_budget": program_fields},
        "supplier": supplier,
        "imports": {"ids": ids, "matching": matches},
    }
    # Bound projections before writing the full comparison artifact.
    require(len(json.dumps(facts, ensure_ascii=True, allow_nan=False).encode("utf-8"))
            <= MAX_STDOUT_BYTES - 4096, "Projected evidence exceeds 4 MiB; nothing was truncated")
    with output.open("xb", buffering=256 * 1024) as destination:
        sink = Sink(destination)
        canonical(world, sink, equipment if args.ignore_maintenance_plan else None)
        sink.flush()
    require(source.stat().st_size == before_bytes and file_hash(source) == before_hash,
            "Input archive changed while it was read")
    facts["canonical"] = {
        "path": str(output), "bytes": sink.bytes, "sha256": sink.digest.hexdigest(),
        "format": FORMAT,
        "ignored_paths": [["nations", index, "equipment", "maintenance_plan"]]
        if args.ignore_maintenance_plan else [],
    }
    payload = json.dumps(facts, ensure_ascii=True, allow_nan=False, separators=(",", ":"))
    require(len(payload.encode("utf-8")) <= MAX_STDOUT_BYTES, "Final projected evidence exceeds 4 MiB")
    sys.stdout.write(payload + "\n")


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        # Never ask an assertion formatter to print a full native campaign.
        sys.stderr.write(json.dumps({"error_type": type(error).__name__,
                                     "error": str(error)[:2000]}, ensure_ascii=True) + "\n")
        sys.exit(1)
