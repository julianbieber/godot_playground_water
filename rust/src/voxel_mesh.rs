use godot::{
    builtin::{
        PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant, VariantArray, Vector2,
        Vector3,
    },
    engine::{
        mesh::{ArrayType, PrimitiveType},
        ArrayMesh, GeometryInstance3D,
    },
    meta::ToGodot,
    obj::{EngineEnum, Gd, NewGd},
};

use crate::voxel_storage::Faces;

pub fn blocky(faces: &Faces) -> Gd<ArrayMesh> {
    let mut m = ArrayMesh::new_gd();

    let mut positions = PackedVector3Array::new();
    let mut indices = PackedInt32Array::new();
    let mut normals = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();
    let mut i = 0;
    for &[x, y, z] in faces.top.iter() {
        let y = y as f32 + 1.0;
        let down_left_x = x as f32;
        let down_left_z = z as f32;
        let down_right_x = x as f32 + 1.0;
        let down_right_z = z as f32;
        let up_left_x = x as f32;
        let up_left_z = z as f32 + 1.0;
        let up_right_x = x as f32 + 1.0;
        let up_right_z = z as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(down_left_x, y, down_left_z));
        positions.push(Vector3::new(down_right_x, y, down_right_z));
        positions.push(Vector3::new(up_left_x, y, up_left_z));
        positions.push(Vector3::new(up_right_x, y, up_right_z));
        indices.push(i * 4);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 3);

        for _ in 0..4 {
            normals.push(Vector3::UP);
        }
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));

        i += 1;
    }
    for &[x, y, z] in faces.bottom.iter() {
        let y = y as f32;
        let down_left_x = x as f32;
        let down_left_z = z as f32;
        let down_right_x = x as f32 + 1.0;
        let down_right_z = z as f32;
        let up_left_x = x as f32;
        let up_left_z = z as f32 + 1.0;
        let up_right_x = x as f32 + 1.0;
        let up_right_z = z as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(down_left_x, y, down_left_z));
        positions.push(Vector3::new(down_right_x, y, down_right_z));
        positions.push(Vector3::new(up_left_x, y, up_left_z));
        positions.push(Vector3::new(up_right_x, y, up_right_z));
        indices.push(i * 4);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 3);
        indices.push(i * 4 + 1);

        for _ in 0..4 {
            normals.push(Vector3::DOWN);
        }
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));

        i += 1;
    }
    for &[x, y, z] in faces.left.iter() {
        let x = x as f32;
        let down_left_y = y as f32;
        let down_left_z = z as f32;
        let down_right_y = y as f32 + 1.0;
        let down_right_z = z as f32;
        let up_left_y = y as f32;
        let up_left_z = z as f32 + 1.0;
        let up_right_y = y as f32 + 1.0;
        let up_right_z = z as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(x, down_left_y, down_left_z));
        positions.push(Vector3::new(x, down_right_y, down_right_z));
        positions.push(Vector3::new(x, up_left_y, up_left_z));
        positions.push(Vector3::new(x, up_right_y, up_right_z));
        indices.push(i * 4);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 3);

        for _ in 0..4 {
            normals.push(Vector3::LEFT);
        }

        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));
        i += 1;
    }
    for &[x, y, z] in faces.right.iter() {
        let x = x as f32 + 1.0;
        let down_left_y = y as f32;
        let down_left_z = z as f32;
        let down_right_y = y as f32 + 1.0;
        let down_right_z = z as f32;
        let up_left_y = y as f32;
        let up_left_z = z as f32 + 1.0;
        let up_right_y = y as f32 + 1.0;
        let up_right_z = z as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(x, down_left_y, down_left_z));
        positions.push(Vector3::new(x, down_right_y, down_right_z));
        positions.push(Vector3::new(x, up_left_y, up_left_z));
        positions.push(Vector3::new(x, up_right_y, up_right_z));
        indices.push(i * 4);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 3);
        indices.push(i * 4 + 1);

        for _ in 0..4 {
            normals.push(Vector3::RIGHT);
        }
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));

        i += 1;
    }
    for &[x, y, z] in faces.back.iter() {
        let z = z as f32 + 1.0;
        let down_left_y = y as f32;
        let down_left_x = x as f32;
        let down_right_y = y as f32 + 1.0;
        let down_right_x = x as f32;
        let up_left_y = y as f32;
        let up_left_x = x as f32 + 1.0;
        let up_right_y = y as f32 + 1.0;
        let up_right_x = x as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(down_left_x, down_left_y, z));
        positions.push(Vector3::new(down_right_x, down_right_y, z));
        positions.push(Vector3::new(up_left_x, up_left_y, z));
        positions.push(Vector3::new(up_right_x, up_right_y, z));
        indices.push(i * 4);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 3);

        for _ in 0..4 {
            normals.push(Vector3::BACK);
        }
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));

        i += 1;
    }
    for &[x, y, z] in faces.front.iter() {
        let z = z as f32;
        let down_left_y = y as f32;
        let down_left_x = x as f32;
        let down_right_y = y as f32 + 1.0;
        let down_right_x = x as f32;
        let up_left_y = y as f32;
        let up_left_x = x as f32 + 1.0;
        let up_right_y = y as f32 + 1.0;
        let up_right_x = x as f32 + 1.0;
        Vector3::new(0.0, 0.0, 0.0).to_variant();

        positions.push(Vector3::new(down_left_x, down_left_y, z));
        positions.push(Vector3::new(down_right_x, down_right_y, z));
        positions.push(Vector3::new(up_left_x, up_left_y, z));
        positions.push(Vector3::new(up_right_x, up_right_y, z));
        indices.push(i * 4);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 3);
        indices.push(i * 4 + 1);

        for _ in 0..4 {
            normals.push(Vector3::FORWARD);
        }
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(0.0, 1.0));
        uvs.push(Vector2::new(1.0, 1.0));

        i += 1;
    }

    let mut variant_array = VariantArray::new();
    variant_array.resize(ArrayType::MAX.ord() as usize, &Variant::nil());
    variant_array.set(ArrayType::VERTEX.ord() as usize, positions.to_variant());
    variant_array.set(ArrayType::INDEX.ord() as usize, indices.to_variant());
    variant_array.set(ArrayType::NORMAL.ord() as usize, normals.to_variant());
    variant_array.set(ArrayType::TEX_UV.ord() as usize, uvs.to_variant());
    m.add_surface_from_arrays(PrimitiveType::TRIANGLES, variant_array);
    m
}
