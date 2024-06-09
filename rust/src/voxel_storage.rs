struct VoxelStorage {
    ground: Vec<u64>,
}

impl VoxelStorage {
    pub fn empty() -> Self {
        VoxelStorage {
            ground: vec![0; 64 * 64], // 64 x 64 x 64 voxels each voxel is 1 bit
        }
    }

    pub fn set(&mut self, coords: [u8; 3]) {
        let lin = linearize_position(coords);
        let height = extract_height(lin);
        let grid_positon = extract_grid_index(lin);
        let ground = self.ground[grid_positon as usize];
        let ground_pattern = (1 as u64) << height;
        let ground = ground | ground_pattern;
        self.ground[grid_positon as usize] = ground;
    }
    pub fn get(&self, coords: [u8; 3]) -> bool {
        let lin = linearize_position(coords);
        let height = extract_height(lin);
        let grid_positon = extract_grid_index(lin);
        let ground = self.ground[grid_positon as usize];
        let ground_pattern = (1 as u64) << height;
        dbg!((ground, ground_pattern));
        (ground & ground_pattern) != 0
    }

    /// returns separate vectors for each side
    pub fn visible_faces(&self) -> Faces {
        let mut faces = Faces::empty();
        for z in 0..64 {
            for x in 0..64 {
                // up/dwon
                let column = self.ground[x + z * 64];
                let bottom_most = (column & 1) == 1;
                if bottom_most {
                    faces.bottom.push([x as u8, 0, z as u8]);
                }
                for y in 0..63u8 {
                    let consecutive = 0b11 as u64;
                    let current_column = column >> y;
                    match current_column & consecutive {
                        0b00 => (),
                        0b10 => faces.bottom.push([x as u8, y + 1, z as u8]),
                        0b01 => faces.top.push([x as u8, y, z as u8]),
                        0b11 => (),
                        _ => panic!(),
                    }
                }
                let top_most = (column & 1 << 63) != 0;
                if top_most {
                    faces.top.push([x as u8, 0, z as u8]);
                }

                // left

                if let Some(left) = VoxelStorage::left([x as u8, z as u8]) {
                    let left_column = self.ground[left[0] as usize + (left[1] as usize) * 64];
                    for y in 0..64u8 {
                        let current_column = left_column >> y;
                        let current_left = left_column >> y;
                        if current_column & 1 == 1 && current_left & 1 == 0 {
                            faces.left.push([x as u8, y, z as u8]);
                        }
                    }
                }
            }
        }
        todo!()
    }

    fn left(p: [u8; 2]) -> Option<[u8; 2]> {
        None
    }
    fn right(p: [u8; 2]) -> Option<[u8; 2]> {
        None
    }
    fn front(p: [u8; 2]) -> Option<[u8; 2]> {
        None
    }
    fn back(p: [u8; 2]) -> Option<[u8; 2]> {
        None
    }
}

pub struct Faces {
    top: Vec<[u8; 3]>,
    bottom: Vec<[u8; 3]>,
    left: Vec<[u8; 3]>,
    right: Vec<[u8; 3]>,
    front: Vec<[u8; 3]>,
    back: Vec<[u8; 3]>,
}

impl Faces {
    fn empty() -> Faces {
        Faces {
            top: Vec::new(),
            bottom: Vec::new(),
            left: Vec::new(),
            right: Vec::new(),
            front: Vec::new(),
            back: Vec::new(),
        }
    }
}

/// position consists of 6 bit for the height, and a 64 * 64 2d grid (12 bit)
/// function assumes valid position
fn delinearize_position(position: u32) -> [u8; 3] {
    let height = extract_height(position);
    let grid_index = extract_grid_index(position);
    let x = (grid_index % 64) as u8;
    let z = (grid_index / 64) as u8;
    [x, height, z]
}

fn extract_height(p: u32) -> u8 {
    (p >> 12) as u8
}

fn extract_grid_index(p: u32) -> u32 {
    p & 0b111111111111
}

/// assumes each of the indices is < 64
fn linearize_position(index: [u8; 3]) -> u32 {
    let height = (index[1] as u32) << 12;
    let grid_index = (index[0] as u32) + (index[2] as u32 * 64);
    height ^ grid_index
}

#[cfg(test)]
mod test {
    use super::{delinearize_position, linearize_position, VoxelStorage};

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

    #[test]
    fn get_and_set_in_chunk() {
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    let mut world = VoxelStorage::empty();
                    assert!(!world.get([x, y, z]));
                    world.set([x, y, z]);
                    assert!(world.get([x, y, z]));
                }
            }
        }
    }

    #[test]
    fn set_full() {
        let mut world = VoxelStorage::empty();
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    assert!(!world.get([x, y, z]));
                    world.set([x, y, z]);
                }
            }
        }
        assert_eq!(world.ground, vec![0xffffffffffffffff; 64 * 64]);
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
