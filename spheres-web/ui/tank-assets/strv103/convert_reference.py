"""Convert the licensed Strv103 Blender source to a self-contained static GLB.

Run only with the official Blender bpy runtime. Never enable source autoexecution.
The original .blend and 4K maps stay outside the repository; see README.md.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import struct
from pathlib import Path

import bpy
import numpy as np
from mathutils import Matrix, Vector
from PIL import Image


SOURCE_HASH = "b2a5742c939b4ad6c153cfe71b01969c3c2bea2673689d5ebf4c624dfe912d03"
SOURCE_URL = "https://opengameart.org/content/stridsvagn-103"
ATTRIBUTION = "Stridsvagn 103 by Lukasz Wesiora (canisferus), CC BY 3.0. " + SOURCE_URL


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def compact_roughness_glb(path):
    """Keep 2K maps; compact only roughness, whose unused R/B channels are free.

    Core glTF uses the green channel for roughness. Metallic factor is zero and
    no occlusion texture uses these maps, so repeating G into R/B changes no
    material meaning and permits efficient high-quality JPEG storage.
    """
    blob = path.read_bytes()
    magic, version, total = struct.unpack_from("<III", blob)
    assert magic == 0x46546C67 and version == 2 and total == len(blob)
    json_size, json_type = struct.unpack_from("<II", blob, 12)
    assert json_type == 0x4E4F534A
    document = json.loads(blob[20:20 + json_size])
    binary_size, binary_type = struct.unpack_from("<II", blob, 20 + json_size)
    assert binary_type == 0x004E4942
    binary = bytearray(blob[28 + json_size:28 + json_size + binary_size])
    assert len(document["buffers"]) == 1 and not document.get("extensionsUsed")
    tangent_repairs = 0
    replacements = {}
    pruned_faces = {}
    def vectors(index, width):
        accessor = document["accessors"][index]
        assert accessor["componentType"] == 5126
        view = document["bufferViews"][accessor["bufferView"]]
        return np.ndarray((accessor["count"], width), dtype="<f4", buffer=binary,
                          offset=view.get("byteOffset", 0) + accessor.get("byteOffset", 0),
                          strides=(view.get("byteStride", width * 4), 4))
    for mesh_index, mesh in enumerate(document["meshes"]):
        for primitive in mesh["primitives"]:
            tangent = vectors(primitive["attributes"]["TANGENT"], 4)
            normals = vectors(primitive["attributes"]["NORMAL"], 3)
            for index in np.flatnonzero(np.linalg.norm(tangent[:, :3], axis=1) < 1e-6):
                # Two source UV-singular corners yield zero exporter tangents.
                # Supply a finite orthogonal basis; keep their original handedness.
                normal = normals[index]
                axis = np.array((1, 0, 0) if abs(normal[0]) < 0.9 else (0, 1, 0), dtype=np.float32)
                replacement = np.cross(normal, axis)
                tangent[index, :3] = replacement / np.linalg.norm(replacement)
                tangent_repairs += 1
            positions = vectors(primitive["attributes"]["POSITION"], 3)
            index_accessor = document["accessors"][primitive["indices"]]
            index_view = document["bufferViews"][index_accessor["bufferView"]]
            index_type = {5123: "<u2", 5125: "<u4"}[index_accessor["componentType"]]
            assert index_accessor.get("byteOffset", 0) == 0 and "byteStride" not in index_view
            indices = np.frombuffer(binary, dtype=index_type, count=index_accessor["count"], offset=index_view.get("byteOffset", 0)).reshape((-1, 3))
            assert indices.nbytes == index_view["byteLength"]
            points = positions[indices]
            cross_length = np.linalg.norm(np.cross(points[:, 1]-points[:, 0], points[:, 2]-points[:, 0]), axis=1)
            keep = cross_length >= 1e-10
            if not np.all(keep):
                source_name = next(node["name"] for node in document["nodes"] if node.get("mesh") == mesh_index)
                pruned_faces[source_name] = pruned_faces.get(source_name, 0) + int(np.count_nonzero(~keep))
                retained = indices[keep].ravel()
                replacements[index_accessor["bufferView"]] = retained.tobytes()
                index_accessor["count"] = len(retained)
                if "min" in index_accessor:
                    index_accessor["min"] = [int(retained.min())]
                if "max" in index_accessor:
                    index_accessor["max"] = [int(retained.max())]
    rough_images = set()
    for material in document["materials"]:
        pbr = material["pbrMetallicRoughness"]
        assert pbr["metallicFactor"] == 0 and "occlusionTexture" not in material
        rough_images.add(document["textures"][pbr["metallicRoughnessTexture"]["index"]]["source"])
    report = []
    for index in rough_images:
        image = document["images"][index]
        view = document["bufferViews"][image["bufferView"]]
        offset = view.get("byteOffset", 0)
        raw = binary[offset:offset + view["byteLength"]]
        with Image.open(io.BytesIO(raw)) as opened:
            green = opened.convert("RGB").getchannel("G")
            output = io.BytesIO()
            Image.merge("RGB", (green, green, green)).save(output, format="JPEG", quality=95, subsampling=0, optimize=True)
        packed = output.getvalue()
        with Image.open(io.BytesIO(packed)) as decoded:
            error = np.asarray(decoded.getchannel("G"), dtype=np.float32) - np.asarray(green, dtype=np.float32)
        rms = float(np.sqrt(np.mean(error ** 2)))
        assert rms < 2.5, "Roughness JPEG fidelity outside conversion budget"
        replacements[image["bufferView"]] = packed
        image["mimeType"] = "image/jpeg"
        report.append({"image": image["name"], "encoding": "JPEG quality=95, 4:4:4, 2048x2048",
                       "roughness_rms_error_8bit": rms, "bytes": len(packed)})
    output_binary = bytearray()
    for index, view in enumerate(document["bufferViews"]):
        while len(output_binary) % 4:
            output_binary.append(0)
        old_offset = view.get("byteOffset", 0)
        payload = replacements.get(index, binary[old_offset:old_offset + view["byteLength"]])
        view["byteOffset"] = len(output_binary)
        view["byteLength"] = len(payload)
        output_binary.extend(payload)
    document["buffers"][0]["byteLength"] = len(output_binary)
    while len(output_binary) % 4:
        output_binary.append(0)
    output_json = json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf8")
    output_json += b" " * ((-len(output_json)) % 4)
    result = (struct.pack("<III", magic, version, 28 + len(output_json) + len(output_binary))
              + struct.pack("<II", len(output_json), json_type) + output_json
              + struct.pack("<II", len(output_binary), binary_type) + output_binary)
    path.write_bytes(result)
    return report, tangent_repairs, pruned_faces


def convert_maps(source, work):
    """Asset conversion: resize source maps and translate gloss to roughness."""
    report = []
    for prefix in ("Body", "Details"):
        paths = {}
        for suffix, kind in (("d", "base_color"), ("n", "normal"), ("g", "roughness")):
            original = source / f"{prefix}_{suffix}.jpg"
            with Image.open(original) as opened:
                assert opened.size == (4096, 4096) and opened.mode == "RGB"
                resized = opened.resize((2048, 2048), Image.Resampling.LANCZOS)
            destination = work / f"{prefix.lower()}-{kind}.{'jpg' if suffix == 'd' else 'png'}"
            if suffix == "d":
                resized.save(destination, quality=92, subsampling=0, optimize=True)
            elif suffix == "n":
                # Keep the source +Y/OpenGL tangent convention. Renormalize after resizing.
                normal = np.asarray(resized, dtype=np.float32) / 127.5 - 1.0
                normal /= np.maximum(np.linalg.norm(normal, axis=2, keepdims=True), 1e-6)
                Image.fromarray(np.clip((normal + 1.0) * 127.5, 0, 255).round().astype(np.uint8)).save(destination, optimize=True)
            else:
                gloss = np.asarray(resized.convert("L"), dtype=np.float32) / 255.0
                # A conservative dielectric approximation of the legacy Cycles gloss mask.
                roughness = np.clip(1.0 - gloss, 0.20, 0.95)
                Image.fromarray(np.round(roughness * 255).astype(np.uint8)).save(destination, optimize=True)
            paths[kind] = destination
            report.append({"material": prefix, "semantic": kind, "source": original.name,
                           "source_sha256": digest(original), "converted": destination.name,
                           "converted_sha256": digest(destination), "dimensions": [2048, 2048]})
        material = bpy.data.materials.new(f"{prefix} - converted painted dielectric")
        material.use_nodes = True
        material.diffuse_color = (0.2, 0.25, 0.13, 1)
        nodes = material.node_tree.nodes
        nodes.clear()
        output = nodes.new("ShaderNodeOutputMaterial")
        principled = nodes.new("ShaderNodeBsdfPrincipled")
        principled.inputs["Metallic"].default_value = 0
        principled.inputs["Roughness"].default_value = 0.7
        principled.inputs["IOR"].default_value = 1.5
        material.node_tree.links.new(principled.outputs["BSDF"], output.inputs["Surface"])
        for semantic, path in paths.items():
            texture = nodes.new("ShaderNodeTexImage")
            texture.image = bpy.data.images.load(str(path), check_existing=False)
            texture.image.colorspace_settings.name = "sRGB" if semantic == "base_color" else "Non-Color"
            texture.interpolation = "Linear"
            if semantic == "normal":
                normal_node = nodes.new("ShaderNodeNormalMap")
                normal_node.space = "TANGENT"
                normal_node.uv_map = "UVMap"
                normal_node.inputs["Strength"].default_value = 1
                material.node_tree.links.new(texture.outputs["Color"], normal_node.inputs["Color"])
                material.node_tree.links.new(normal_node.outputs["Normal"], principled.inputs["Normal"])
            else:
                material.node_tree.links.new(texture.outputs["Color"], principled.inputs["Base Color" if semantic == "base_color" else "Roughness"])
        material["source_material"] = prefix
        material["conversion"] = "2K diffuse; renormalized OpenGL normal; roughness=clamp(1-gloss,0.20,0.95); metallic=0. Legacy colored specular omitted."
        yield prefix, material, report[-3:]


def render_preview(scene, destination):
    """Render the converted scene, not an author screenshot, outside the repo."""
    scene.render.engine = "CYCLES"
    scene.cycles.device = "CPU"
    scene.cycles.samples = 32
    scene.cycles.use_denoising = True
    scene.render.resolution_x = 1400
    scene.render.resolution_y = 1000
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.filepath = str(destination)
    scene.world = bpy.data.worlds.new("Reference studio")
    scene.world.use_nodes = True
    scene.world.node_tree.nodes["Background"].inputs["Color"].default_value = (0.35, 0.42, 0.5, 1)
    scene.world.node_tree.nodes["Background"].inputs["Strength"].default_value = 0.35
    scene.view_settings.view_transform = "AgX"
    bpy.ops.mesh.primitive_plane_add(size=200, location=(0, 0, -0.015))
    floor = bpy.context.object
    floor.name = "QA-only studio floor (not exported)"
    mat = bpy.data.materials.new("Studio floor")
    mat.diffuse_color = (0.17, 0.19, 0.2, 1)
    floor.data.materials.append(mat)
    for name, location, power, size in (("Key", (-4, -6, 10), 1800, 7), ("Fill", (7, -1, 6), 1100, 6), ("Rim", (-3, 7, 8), 2100, 5)):
        light = bpy.data.lights.new(name, "AREA")
        light.energy = power
        light.shape = "DISK"
        light.size = size
        obj = bpy.data.objects.new(name, light)
        scene.collection.objects.link(obj)
        obj.location = location
        obj.rotation_euler = (Vector((0, 0, 1)) - obj.location).to_track_quat("-Z", "Y").to_euler()
    camera_data = bpy.data.cameras.new("Reference QA camera")
    camera = bpy.data.objects.new("Reference QA camera", camera_data)
    scene.collection.objects.link(camera)
    camera.location = (9.5, -12.5, 8.0)
    camera.rotation_euler = (Vector((0, -0.3, 1.1)) - camera.location).to_track_quat("-Z", "Y").to_euler()
    camera_data.type = "ORTHO"
    camera_data.ortho_scale = 11.5
    scene.camera = camera
    bpy.ops.render.render(write_still=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path, help="Official extracted stridsvagn103 directory")
    parser.add_argument("output", type=Path)
    parser.add_argument("--work", type=Path, required=True, help="Temporary map conversion directory outside repo")
    parser.add_argument("--preview", type=Path)
    args = parser.parse_args()
    source = args.source.resolve()
    args.work = args.work.resolve()
    args.output = args.output.resolve()
    if args.preview:
        args.preview = args.preview.resolve()
    blend = source / "stridsvagn103.blend"
    assert digest(blend) == SOURCE_HASH, "Unexpected source file; review provenance before conversion"
    args.work.mkdir(parents=True, exist_ok=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    bpy.context.preferences.filepaths.use_scripts_auto_execute = False
    bpy.ops.wm.open_mainfile(filepath=str(blend), load_ui=False, use_scripts=False)
    assert not bpy.context.preferences.filepaths.use_scripts_auto_execute
    assert bpy.app.background
    assert len(bpy.data.texts) == 0, "Unexpected embedded source text blocks"
    scene = bpy.context.scene
    original_units = {"system": scene.unit_settings.system, "scale_length": scene.unit_settings.scale_length}
    meshes = [obj for obj in scene.objects if obj.type == "MESH"]
    world_points = [obj.matrix_world @ vertex.co for obj in meshes for vertex in obj.data.vertices]
    low = Vector(tuple(min(point[axis] for point in world_points) for axis in range(3)))
    high = Vector(tuple(max(point[axis] for point in world_points) for axis in range(3)))
    translation = Vector((-(low.x + high.x) / 2, -(low.y + high.y) / 2, -low.z))
    depsgraph = bpy.context.evaluated_depsgraph_get()
    object_report = []
    converted = dict()
    textures = []
    for prefix, material, texture_report in convert_maps(source, args.work):
        converted[prefix] = material
        textures.extend(texture_report)
    bpy.ops.object.select_all(action="DESELECT")
    for obj in meshes:
        old_material_names = [slot.material.name for slot in obj.material_slots]
        evaluated = obj.evaluated_get(depsgraph)
        mesh = bpy.data.meshes.new_from_object(evaluated, preserve_all_data_layers=True, depsgraph=depsgraph)
        assert mesh.uv_layers.active and len(mesh.uv_layers.active.data) == len(mesh.loops)
        matrix = Matrix.Translation(translation) @ obj.matrix_world
        assert matrix.determinant() > 0, "Unexpected reflected source object"
        normal_matrix = matrix.to_3x3().inverted().transposed()
        corner_normals = [(normal_matrix @ normal.vector).normalized() for normal in mesh.corner_normals]
        mesh.transform(matrix)
        mesh.update()
        mesh.normals_split_custom_set(corner_normals)
        obj.data = mesh
        obj.matrix_world = Matrix.Identity(4)
        mesh.materials.clear()
        for material_name in old_material_names:
            mesh.materials.append(converted[material_name])
        mesh.calc_loop_triangles()
        object_report.append({"source_name": obj.name, "triangles": len(mesh.loop_triangles),
                              "source_vertices": len(mesh.vertices), "materials": old_material_names,
                              "uv_layer": mesh.uv_layers.active.name, "world_transform_baked": True})
        obj.select_set(True)
        obj["source_attribution"] = ATTRIBUTION
    scene["reference_only"] = "Historical Stridsvagn 103; not a configurable generic tank or gameplay equipment mapping."
    scene["source"] = SOURCE_URL
    scene["license"] = "CC-BY-3.0"
    scene["source_sha256"] = SOURCE_HASH
    bpy.ops.export_scene.gltf(filepath=str(args.output.resolve()), export_format="GLB",
        export_copyright=ATTRIBUTION, export_image_format="AUTO", export_jpeg_quality=92,
        use_selection=True, export_texcoords=True, export_normals=True, export_tangents=True,
        export_materials="EXPORT", export_extras=True, export_yup=True, export_apply=False,
        export_animations=False, export_skins=False, export_morph=False, export_cameras=False,
        export_lights=False, export_vertex_color="NONE", export_draco_mesh_compression_enable=False,
        export_meshopt_compression_enable=False, export_use_gltfpack=False)
    roughness_encoding, tangent_repairs, pruned_faces = compact_roughness_glb(args.output)
    for row in object_report:
        row["source_triangles"] = row["triangles"]
        row["degenerate_faces_removed"] = pruned_faces.get(row["source_name"], 0)
        row["triangles"] -= row["degenerate_faces_removed"]
    report = {"version": 1, "asset": args.output.name, "source": SOURCE_URL,
              "source_archive": "stridsvagn103_0.7z", "source_blend_sha256": SOURCE_HASH,
              "author": "Lukasz Wesiora (canisferus)", "license": "CC-BY-3.0",
              "license_url": "https://creativecommons.org/licenses/by/3.0/",
              "bpy_version": bpy.app.version_string, "source_autoexecution": False,
              "source_units": original_units, "scale_policy": "Source numeric scale retained; physical meters not independently calibrated.",
              "source_bounds_blender_z_up": {"min": list(low), "max": list(high)},
              "translation_blender": list(translation),
              "glb_coordinates": "Right-handed Y-up; Blender (x,y,z) becomes glTF (x,z,-y); cannon faces +Z. Ground Y=0.",
              "triangles": sum(row["triangles"] for row in object_report), "mesh_objects": len(meshes),
              "source_triangles": sum(row["source_triangles"] for row in object_report),
              "degenerate_faces_removed": sum(pruned_faces.values()),
              "degenerate_filter": "Remove indexed triangles whose world-coordinate edge cross-product length is below 1e-10 (twice area in preserved source units). Vertex/UV/normal streams retained.",
              "objects": object_report, "textures": textures,
              "embedded_roughness_encoding": roughness_encoding,
              "zero_tangent_corners_repaired": tangent_repairs,
              "material_conversion": "Dielectric metallic=0, IOR=1.5; source diffuse is sRGB; tangent OpenGL normals renormalized after resizing; roughness=clamp(1-gloss,0.20,0.95). Legacy colored specular/glass mixture cannot be represented exactly by core metallic-roughness and is omitted.",
              "animation": "Static evaluated meshes; no rig, animations, moving track simulation or turret.",
              "bytes": args.output.stat().st_size, "sha256": digest(args.output)}
    args.output.with_suffix(".report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf8")
    print(json.dumps({"output": str(args.output), "triangles": report["triangles"], "bytes": report["bytes"], "sha256": report["sha256"]}))
    if args.preview:
        render_preview(scene, args.preview.resolve())


if __name__ == "__main__":
    main()
