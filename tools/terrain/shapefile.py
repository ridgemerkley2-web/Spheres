"""shapefile.py — a minimal, dependency-free ESRI shapefile reader.

WHY THIS EXISTS. The bakers in this directory read GeoJSON because that is what
was on disk. Natural Earth publishes its 10m data as shapefiles at a fraction of
the GeoJSON size — ne_10m_lakes is 2.3 MB zipped against 25 MB of GeoJSON — and
no shapefile library is installed here (pyshp, fiona, geopandas and GDAL are all
absent; shapely, numpy, scipy and PIL are present). The format is simple enough
that reading it directly is smaller than adding a dependency, and it keeps these
bakers reproducible from nothing but a Python install.

WHAT IT READS. Point (1), PolyLine (3), Polygon (5) and their Z/M variants,
which is every shape type Natural Earth ships. Records come back as a list of
parts, each part a list of (lon, lat) tuples, exactly the structure the existing
bakers already walk. It also reads the .dbf attribute table, which is where
Natural Earth keeps names and scale ranks.

WHAT IT DOES NOT DO. No projection handling (Natural Earth 10m is WGS84 lon/lat,
which is what every baker here assumes and what the .prj states), no writing, no
spatial index, no M-value interpretation. It is a reader for these files and not
a general library.

The specification is public: ESRI Shapefile Technical Description, July 1998.
Byte layout, in short — a 100-byte header, then records, each an 8-byte big
endian header (record number, content length in 16-bit words) followed by a
little endian shape record: type (int32), and for polyline/polygon a bounding
box (4 doubles), part count (int32), point count (int32), part start indices
(int32 each), then the points (two doubles each).
"""

import struct


def _dbf_fields(data):
    """Field descriptors from a .dbf header: (name, type, length)."""
    count = (struct.unpack("<H", data[8:10])[0] - 33) // 32
    out = []
    for i in range(count):
        off = 32 + i * 32
        name = data[off:off + 11].split(b"\0")[0].decode("latin-1").strip()
        kind = chr(data[off + 11])
        size = data[off + 16]
        out.append((name, kind, size))
    return out


def _encoding_for(path):
    """The codepage a shapefile declares in its sibling .cpg, which Natural
    Earth ships and which is UTF-8. Reading the .dbf as latin-1 instead turns
    Vaenern into a name no filter matches, and the failure is silent because
    every other name is ASCII. Falls back to latin-1, which cannot raise."""
    cpg = path[:-4] + ".cpg" if path.lower().endswith(".dbf") else path + ".cpg"
    try:
        with open(cpg, "r") as fh:
            name = fh.read().strip()
        "".encode(name)                       # reject a codepage Python lacks
        return name
    except Exception:
        return "latin-1"


def read_dbf(path):
    """The whole attribute table as a list of dicts. Values stay strings except
    numeric fields, which become int or float; blanks become None."""
    enc = _encoding_for(path)
    with open(path, "rb") as fh:
        data = fh.read()
    records, header_len, record_len = struct.unpack("<IHH", data[4:12])
    fields = _dbf_fields(data)
    out = []
    for r in range(records):
        base = header_len + r * record_len
        if data[base:base + 1] == b"*":          # deleted
            continue
        row, off = {}, base + 1
        for name, kind, size in fields:
            # NUL padding as well as spaces: Natural Earth pads its character
            # fields with \0, and a value read as "River\0\0\0" compares equal
            # to nothing at all. That silently emptied a filter here once.
            raw = data[off:off + size].decode(enc, "replace").replace("\0", " ").strip()
            off += size
            if kind in "NF":
                row[name] = None if raw in ("", "-") else (float(raw) if "." in raw or "e" in raw.lower() else int(raw))
            else:
                row[name] = raw
        out.append(row)
    return out


def read_shp(path):
    """Every record as {"type": int, "parts": [[(lon, lat), ...], ...]}.

    Point records come back as a single one-point part so callers can treat
    every record the same way.
    """
    with open(path, "rb") as fh:
        data = fh.read()
    if struct.unpack(">i", data[0:4])[0] != 9994:
        raise ValueError(path + ": not a shapefile (bad magic)")
    total = struct.unpack(">i", data[24:28])[0] * 2      # file length, in bytes
    out, pos = [], 100
    while pos < min(total, len(data)):
        _, words = struct.unpack(">ii", data[pos:pos + 8])
        body = pos + 8
        pos = body + words * 2
        kind = struct.unpack("<i", data[body:body + 4])[0]
        if kind == 0:                                    # null shape
            out.append({"type": 0, "parts": []})
            continue
        if kind in (1, 11, 21):                          # point / pointZ / pointM
            x, y = struct.unpack("<dd", data[body + 4:body + 20])
            out.append({"type": kind, "parts": [[(x, y)]]})
            continue
        if kind not in (3, 5, 13, 15, 23, 25):           # polyline / polygon + Z/M
            out.append({"type": kind, "parts": []})
            continue
        n_parts, n_points = struct.unpack("<ii", data[body + 36:body + 44])
        p = body + 44
        starts = list(struct.unpack("<%di" % n_parts, data[p:p + 4 * n_parts]))
        p += 4 * n_parts
        flat = struct.unpack("<%dd" % (2 * n_points), data[p:p + 16 * n_points])
        parts = []
        for i, s in enumerate(starts):
            e = starts[i + 1] if i + 1 < len(starts) else n_points
            parts.append([(flat[2 * j], flat[2 * j + 1]) for j in range(s, e)])
        out.append({"type": kind, "parts": parts})
    return out


def read(path_without_extension):
    """Geometry and attributes together, zipped into one list of dicts with
    "parts" alongside every .dbf column."""
    shapes = read_shp(path_without_extension + ".shp")
    rows = read_dbf(path_without_extension + ".dbf")
    if len(shapes) != len(rows):
        raise ValueError("%s: %d shapes against %d attribute rows"
                         % (path_without_extension, len(shapes), len(rows)))
    out = []
    for shape, row in zip(shapes, rows):
        merged = dict(row)
        merged["parts"] = shape["parts"]
        merged["shape_type"] = shape["type"]
        out.append(merged)
    return out
