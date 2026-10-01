"""Read the identity and exception streams of a retained Windows dump offline."""
import argparse
import hashlib
import json
import mmap
from pathlib import Path
import struct


def inspect(path):
    # Map header/stream tables only as accessed; never allocate a full dump in RAM.
    with path.open("rb") as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as b:
        result = inspect_mapped(b)
        source.seek(0)
        digest = hashlib.file_digest(source, "sha256").hexdigest()
    return dict(path=str(path.resolve()), bytes=path.stat().st_size, sha256=digest, **result)


def inspect_mapped(b):
    signature, version, count, directory, checksum, stamp, flags = struct.unpack_from("<IIIIIIQ", b)
    if signature != 0x504D444D:
        raise ValueError("Not an MDMP file")
    streams = {}
    for i in range(count):
        typ, size, rva = struct.unpack_from("<III", b, directory + i * 12)
        if rva + size > len(b):
            raise ValueError("Stream exceeds dump bounds")
        streams[typ] = {"bytes": size, "rva": rva}
    exc = streams[6]["rva"]
    thread = struct.unpack_from("<I", b, exc)[0]
    code, exc_flags, record, address, nparams = struct.unpack_from("<IIQQI", b, exc + 8)
    if nparams > 15:
        raise ValueError("Invalid exception parameter count")
    params = struct.unpack_from("<" + "Q" * nparams, b, exc + 40)
    context_bytes, context_rva = struct.unpack_from("<II", b, exc + 160)
    misc = streams[15]["rva"]
    misc_size, misc_flags, pid = struct.unpack_from("<III", b, misc)
    if not misc_flags & 1:
        raise ValueError("Process identity absent")
    mem = streams[9]["rva"]
    ranges, payload = struct.unpack_from("<QQ", b, mem)
    if 16 + ranges * 16 > streams[9]["bytes"]:
        raise ValueError("Full memory range table exceeds its stream")
    memory_bytes = sum(struct.unpack_from("<Q", b, mem + 16 + i * 16 + 8)[0] for i in range(ranges))
    if not memory_bytes or payload + memory_bytes > len(b):
        raise ValueError("Full memory payload is truncated or absent")
    sys = streams[7]["rva"]
    arch = struct.unpack_from("<H", b, sys)[0]
    major, minor, build = struct.unpack_from("<III", b, sys + 8)
    mods = streams[4]["rva"]
    if 4 + struct.unpack_from("<I", b, mods)[0] * 108 > streams[4]["bytes"]:
        raise ValueError("Module table exceeds its stream")
    modules = []
    for i in range(struct.unpack_from("<I", b, mods)[0]):
        z = mods + 4 + i * 108
        base, size, check, timestamp, name_rva = struct.unpack_from("<QIIII", b, z)
        name_bytes = struct.unpack_from("<I", b, name_rva)[0]
        name = b[name_rva + 4:name_rva + 4 + name_bytes].decode("utf-16le")
        ms, ls = struct.unpack_from("<II", b, z + 32)
        modules.append(dict(name=name, base=hex(base), bytes=size,
                            version=f"{ms >> 16}.{ms & 65535}.{ls >> 16}.{ls & 65535}"))
    return dict(flags=hex(flags), full_memory=bool(flags & 2), process_id=pid,
                memory64_payload_complete=True, memory64_bytes=memory_bytes,
                exception=dict(thread_id=thread, code=hex(code), flags=exc_flags,
                               address=hex(address), parameters=list(params),
                               context_bytes=context_bytes, context_rva=context_rva),
                os=dict(architecture=arch, major=major, minor=minor, build=build),
                streams=streams, modules=modules)


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("dump", type=Path)
    p.add_argument("output", type=Path)
    args = p.parse_args()
    result = inspect(args.dump)
    with args.output.open("x", encoding="utf8") as out:
        json.dump(result, out, indent=2)
        out.write("\n")
    print(json.dumps({k: v for k, v in result.items() if k not in ("streams", "modules")}, indent=2))
