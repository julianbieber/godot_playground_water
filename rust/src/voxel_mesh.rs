use godot::{
    engine::ArrayMesh,
    obj::{Gd, NewGd},
};

use crate::voxel_storage::Faces;

pub fn blocky(faces: &Faces) -> Gd<ArrayMesh> {
    let mut m = ArrayMesh::new_gd();

    let mut positions: Vec<f32> = Vec::with_capacity(faces.total() * 4 * 3);
    let mut indices: Vec<i32> = Vec::with_capacity(faces.total() * 6);
    for (i, &[x, y, z]) in faces.top.iter().enumerate() {
        let y = y as f32 + 1.0;
        let down_left_x = x as f32;
        let down_left_z = z as f32;
        let down_right_x = x as f32 + 1.0;
        let down_right_z = z as f32;
        let up_left_x = x as f32;
        let up_left_z = z as f32 + 1.0;
        let up_right_x = x as f32 + 1.0;
        let up_right_z = z as f32 + 1.0;
        positions.extend_from_slice(&[
            down_left_x,
            y,
            down_left_z,
            down_right_x,
            y,
            down_right_z,
            up_left_x,
            y,
            up_left_z,
            up_right_x,
            y,
            up_right_z,
        ]);
        let i = i as i32;
        indices.extend_from_slice(&[i * 6, i * 6 + 1, i * 6 + 2, i * 6 + 2, i * 6 + 1, i * 6 + 3])
    }

    m
}
