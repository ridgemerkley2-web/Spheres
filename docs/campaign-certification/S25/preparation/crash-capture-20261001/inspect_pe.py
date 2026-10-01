"""Read-only PE/COFF inspection; no binary execution, mutation or dependencies."""
import argparse
import hashlib
import json
from pathlib import Path
import struct


def inspect(path):
    b = path.read_bytes()
    pe = struct.unpack_from("<I", b, 0x3C)[0]
    assert b[:2] == b"MZ" and b[pe:pe+4] == b"PE\0\0"
    machine, count, stamp, symptr, nsyms, optsz, flags = struct.unpack_from("<HHIIIHH", b, pe + 4)
    opt = pe + 24
    strings = symptr + nsyms * 18

    def zstr(offset):
        end = b.find(b"\0", offset)
        return b[offset:end].decode("utf8", "replace")

    sections = []
    for i in range(count):
        z = opt + optsz + i * 40
        name = b[z:z+8].rstrip(b"\0").decode("ascii", "replace")
        if name.startswith("/") and name[1:].isdigit():
            name = zstr(strings + int(name[1:]))
        virtual_size, rva, size, offset = struct.unpack_from("<IIII", b, z + 8)
        sections.append(dict(name=name, virtual_size=virtual_size, rva=rva, bytes=size, offset=offset))

    def rva_offset(rva):
        for s in sections:
            if s["rva"] <= rva < s["rva"] + max(s["virtual_size"], s["bytes"]):
                return s["offset"] + rva - s["rva"]
        raise ValueError("Unmapped RVA")

    magic = struct.unpack_from("<H", b, opt)[0]
    dirs = opt + (112 if magic == 0x20B else 96)
    debug_rva, debug_size = struct.unpack_from("<II", b, dirs + 6 * 8)
    debug = []
    if debug_rva:
        base = rva_offset(debug_rva)
        for i in range(debug_size // 28):
            _, _, _, _, typ, size, addr, raw = struct.unpack_from("<IIHHIIII", b, base + i * 28)
            debug.append(dict(type=typ, bytes=size, signature=b[raw:raw+4].hex(),
                              pdb_path=zstr(raw+24) if b[raw:raw+4] == b"RSDS" else None))
    names = []
    i = 0
    while i < nsyms:
        z = symptr + i * 18
        raw = b[z:z+8]
        name = zstr(strings + struct.unpack_from("<I", raw, 4)[0]) if raw[:4] == b"\0" * 4 else raw.rstrip(b"\0").decode("utf8", "replace")
        value, section, typ, cls, aux = struct.unpack_from("<IhHBB", b, z + 8)
        if "s25_stability_cell" in name:
            names.append(dict(name=name, section=section, value=value))
        i += 1 + aux
    return dict(path=str(path.resolve()), bytes=len(b), sha256=hashlib.sha256(b).hexdigest(),
                machine=hex(machine), coff_symbols=nsyms, sections=sections,
                debug_entries=debug, selected_symbols=names,
                contains_compiled_revision=b"68ba0622ec709b78617aadd1f9198d18f532bb32" in b)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = inspect(args.binary)
    with args.output.open("x", encoding="utf8") as out:
        json.dump(result, out, indent=2)
        out.write("\n")
    print(json.dumps(result, indent=2))
