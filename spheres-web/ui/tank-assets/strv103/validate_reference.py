"""Offline structural/geometry/image validation for the shipped reference GLB."""
import hashlib
import io
import json
import struct
from pathlib import Path

import numpy as np
from PIL import Image


def main():
    path = Path(__file__).with_name("strv103.glb")
    blob = path.read_bytes()
    report = json.loads(path.with_suffix(".report.json").read_text(encoding="utf8"))
    assert len(blob) == report["bytes"] < 15_000_000
    assert hashlib.sha256(blob).hexdigest() == report["sha256"]
    assert struct.unpack_from("<III", blob) == (0x46546C67, 2, len(blob))
    n, kind = struct.unpack_from("<II", blob, 12)
    assert kind == 0x4E4F534A and n % 4 == 0
    doc = json.loads(blob[20:20+n])
    size, kind = struct.unpack_from("<II", blob, 20+n)
    assert kind == 0x004E4942 and size % 4 == 0
    binary = blob[28+n:28+n+size]
    assert len(binary) == size and len(blob) == 28+n+size
    assert doc["asset"]["version"] == "2.0"
    assert "Lukasz Wesiora" in doc["asset"]["copyright"] and "CC BY 3.0" in doc["asset"]["copyright"]
    assert not doc.get("extensionsUsed") and not doc.get("extensionsRequired")
    assert not doc.get("animations") and not doc.get("skins") and not doc.get("cameras")
    assert len(doc["buffers"]) == 1 and "uri" not in doc["buffers"][0]
    assert 0 <= size - doc["buffers"][0]["byteLength"] <= 3
    for view in doc["bufferViews"]:
        assert view.get("buffer", 0) == 0
        assert view.get("byteOffset", 0) % 4 == 0
        assert view.get("byteOffset", 0) + view["byteLength"] <= len(binary)

    def accessor(index):
        acc = doc["accessors"][index]
        assert not acc.get("sparse") and not acc.get("normalized")
        view = doc["bufferViews"][acc["bufferView"]]
        scalar = {5121: "u1", 5123: "<u2", 5125: "<u4", 5126: "<f4"}[acc["componentType"]]
        width = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}[acc["type"]]
        item_size = np.dtype(scalar).itemsize
        stride = view.get("byteStride", width * item_size)
        offset = view.get("byteOffset", 0) + acc.get("byteOffset", 0)
        assert acc.get("byteOffset", 0) + (acc["count"]-1) * stride + width * item_size <= view["byteLength"]
        data = np.ndarray((acc["count"], width), dtype=scalar, buffer=binary, offset=offset, strides=(stride, item_size))
        assert np.isfinite(data).all()
        return data

    triangles = 0
    vertices = 0
    positions = []
    degenerates = 0
    for mesh in doc["meshes"]:
        for primitive in mesh["primitives"]:
            assert primitive.get("mode", 4) == 4 and not primitive.get("targets")
            attrs = primitive["attributes"]
            assert all(key in attrs for key in ("POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"))
            pos, normal, uv, tangent = (accessor(attrs[key]) for key in ("POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"))
            assert len(pos) == len(normal) == len(uv) == len(tangent)
            assert np.all(np.abs(np.linalg.norm(normal, axis=1)-1) < 0.002)
            assert np.all(np.abs(np.linalg.norm(tangent[:, :3], axis=1)-1) < 0.002)
            assert np.all(np.abs(tangent[:, 3]) == 1)
            indices = accessor(primitive["indices"]).ravel()
            assert len(indices) % 3 == 0 and indices.max() < len(pos)
            corners = pos[indices].reshape((-1, 3, 3))
            area = np.linalg.norm(np.cross(corners[:, 1]-corners[:, 0], corners[:, 2]-corners[:, 0]), axis=1)
            degenerates += int(np.count_nonzero(area < 1e-10))
            triangles += len(indices)//3
            vertices += len(pos)
            positions.append(pos)
            assert primitive["material"] < len(doc["materials"])
    assert triangles == report["triangles"] == 25675
    assert degenerates == 0
    assert len(doc["meshes"]) == report["mesh_objects"] == 16
    assert len(doc["materials"]) == 2 and len(doc["images"]) == 6
    for node in doc["nodes"]:
        assert not any(key in node for key in ("matrix", "translation", "rotation", "scale"))
    all_positions = np.concatenate(positions)
    low, high = all_positions.min(axis=0), all_positions.max(axis=0)
    assert abs(float(low[1])) < 1e-5, "GLB must rest on Y=0"
    assert np.allclose(high-low, [3.543793, 2.752504, 8.951853], atol=1e-4)
    textures = []
    for image in doc["images"]:
        assert "uri" not in image and image["mimeType"] in ("image/jpeg", "image/png")
        view = doc["bufferViews"][image["bufferView"]]
        start = view.get("byteOffset", 0)
        raw = binary[start:start+view["byteLength"]]
        with Image.open(io.BytesIO(raw)) as decoded:
            decoded.load()
            assert decoded.size == (2048, 2048) and decoded.mode == "RGB"
            assert decoded.format == ("PNG" if image["mimeType"] == "image/png" else "JPEG")
        if "normal" in image["name"]:
            assert image["mimeType"] == "image/png"
        textures.append({"name": image["name"], "mime_type": image["mimeType"], "dimensions": [2048, 2048],
                         "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()})
    for material in doc["materials"]:
        pbr = material["pbrMetallicRoughness"]
        assert pbr["metallicFactor"] == 0
        assert all("index" in item for item in (pbr["baseColorTexture"], pbr["metallicRoughnessTexture"], material["normalTexture"]))
    result = {"status": "pass", "sha256": report["sha256"], "bytes": len(blob), "triangles": triangles,
              "exported_vertices_including_uv_and_normal_splits": vertices, "meshes": 16, "materials": 2,
              "degenerate_triangles_below_1e-10_cross_length": degenerates,
              "bounds_gltf_y_up": {"min": low.tolist(), "max": high.tolist()},
              "external_resources": 0, "required_extensions": [], "images": textures}
    path.with_suffix(".validation.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
