struct VoxelStorage {
    ground: Vec<u64>,
}

impl VoxelStorage {
    fn empty() -> Self {
        VoxelStorage {
            ground: vec![0; 64 * 64], // 64 x 64 x 64 voxels each voxel is 1 bit
        }
    }

    fn set(&mut self, coords: [u8; 3], value: bool) {
        let mut yz: usize = 0;
        yz |= (coords[2] as usize) << 6;
        yz |= (coords[1] as usize);
    }
}

/// position consists of 6 bit for the height, and a 64 * 64 2d grid (12 bit)
/// function assumes valid position
fn delinearize_position(position: u32) -> [u8; 3] {
    let height = (position >> 12) as u8;
    let grid_index = position & 0b111111111111;
    let x = (grid_index % 64) as u8;
    let z = (grid_index / 64) as u8;
    [x, height, z]
}

/// assumes each of the indices is < 64
fn linearize_position(index: [u8; 3]) -> u32 {
    let height = (index[1] as u32) << 12;
    let grid_index = (index[0] as u32 + 1) * (index[2] as u32);
    height ^ grid_index
}

#[cfg(test)]
mod test {
    use super::{delinearize_position, linearize_position};

    #[test]
    fn check_position_conversion() {
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    dbg!([x, y, z]);
                    let lin = linearize_position([x, y, z]);
                    dbg!(lin);
                    let delin = delinearize_position(lin);
                    assert_eq!(delin, [x, y, z]);
                }
            }
        }
    }
}

// fn create_voxels() -> (PackedVector3Array, PackedInt32Array) {
//     let mut voxels = BitVec::from_elem(32 * 32 * 32, false)
//     for i in 0..ChunkShape::SIZE {
//         let [x, y, z] = ChunkShape::delinearize(i);
//         voxels[i as usize] = if ((x * x + y * y + z * z) as f32).sqrt() < 15.0 {
//             FULL
//         } else {
//             EMPTY
//         };
//     }

//     let mut buffer = GreedyQuadsBuffer::new(voxels.len());
//     greedy_quads(
//         &voxels,
//         &ChunkShape {},
//         [0; 3],
//         [17; 3],
//         &RIGHT_HANDED_Y_UP_CONFIG.faces,
//         &mut buffer,
//     );
//     let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

//     let num_indices = buffer.quads.num_quads() * 6;
//     let num_vertices = buffer.quads.num_quads() * 4;
//     let mut indices = PackedInt32Array::new();
//     indices.resize(num_indices);
//     let mut positions = PackedVector3Array::new();
//     positions.resize(num_vertices);
//     let mut normals = Vec::with_capacity(num_vertices);
//     for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
//         for quad in group.into_iter() {
//             indices.extend(
//                 face.quad_mesh_indices(positions.len() as u32)
//                     .into_iter()
//                     .map(|v| v as i32),
//             );
//             positions.extend(
//                 face.quad_mesh_positions(&quad, 1.0)
//                     .into_iter()
//                     .map(|v| Vector3 {
//                         x: v[0],
//                         y: v[1],
//                         z: v[2],
//                     }),
//             );
//             normals.extend_from_slice(&face.quad_mesh_normals());
//         }
//     }
//     let i_l = indices.len();
//     let p_l = positions.len();
//     godot_print!("length {i_l}, {p_l}");
//     (positions, indices)
// }
