use crate::voxel_storage::{ChunkStorage, VoxelStorage, VoxelWorld};

pub fn simulate_water(chunks: &mut VoxelWorld, step_counter: u8) {
    let mut new_water = ChunkStorage::new();
    for (i, ground) in chunks.ground.iter() {
        let water = &chunks.water[i];
        let mut new_water_element = VoxelStorage::empty();
        for ((&ground_column, &water_column), new_water_column) in ground
            .raw
            .iter()
            .zip(water.raw.iter())
            .zip(new_water_element.raw.iter_mut())
        {
            for offset in 1..64 {
                let cell_selector = 1u64 << offset;
                let water_cell = water_column & cell_selector;
                let down_water_cell = water_cell >> 1;
                let condition =
                    (!ground_column & down_water_cell) & (!*new_water_column & down_water_cell);
                *new_water_column |= (down_water_cell & condition)
                    | (((!condition << 1) & cell_selector) & water_cell);
            }
        }
        new_water.insert(i.clone(), new_water_element);
    }

    let step_counter = step_counter % 8;
    if step_counter < 2 {
        for (i, ground) in chunks.ground.iter() {
            {
                let water = new_water.get_mut(i).unwrap();
                for x in 1..64 {
                    for z in 0..64 {
                        let left_ground_column = ground.get_pillar([x - 1, z]);
                        let left_water_column = water.get_pillar([x - 1, z]);
                        let left_free = (!left_ground_column) & (!left_water_column);
                        let current_water = water.get_pillar([x, z]);
                        let water_flow = left_free & current_water;
                        let left_new_water = left_water_column | water_flow;
                        let new_water = current_water & (!water_flow);
                        water.set_pillar([x - 1, z], left_new_water);
                        water.set_pillar([x, z], new_water);
                    }
                }
            } // edge
            if i[0] > chunks.xs.start {
                let mut left_results = [0u64; 64];
                let mut current_results = [0u64; 64];
                {
                    let left_ground = &chunks.ground[&[i[0] - 1, i[1]]];
                    let left_water = &new_water[&[i[0] - 1, i[1]]];
                    let current_water = &new_water[i];
                    for z in 0..64u8 {
                        let left_ground_column = left_ground.get_pillar([63, z]);
                        let left_water_column = left_water.get_pillar([63, z]);
                        let left_free = (!left_ground_column) & (!left_water_column);
                        let current_water = current_water.get_pillar([0, z]);
                        let water_flow = left_free & current_water;
                        let left_new_water = left_water_column | water_flow;
                        let new_water = current_water & (!water_flow);
                        left_results[z as usize] = left_new_water;
                        current_results[z as usize] = new_water;
                    }
                }
                {
                    let left_water = new_water.get_mut(&[i[0] - 1, i[1]]).unwrap();
                    for (i, v) in left_results.into_iter().enumerate() {
                        left_water.set_pillar([63, i as u8], v);
                    }
                }
                {
                    let current_water = new_water.get_mut(i).unwrap();
                    for (i, v) in current_results.into_iter().enumerate() {
                        current_water.set_pillar([0, i as u8], v);
                    }
                }
            }
        }
    } else if step_counter < 4 {
        for (i, ground) in chunks.ground.iter() {
            let water = new_water.get_mut(i).unwrap();
            for x in 0..63 {
                for z in 0..64 {
                    let left_ground_column = ground.get_pillar([x + 1, z]);
                    let left_water_column = water.get_pillar([x + 1, z]);
                    let left_free = (!left_ground_column) & (!left_water_column);
                    let current_water = water.get_pillar([x, z]);
                    let water_flow = left_free & current_water;
                    let left_new_water = left_water_column | water_flow;
                    let new_water = current_water & (!water_flow);
                    water.set_pillar([x + 1, z], left_new_water);
                    water.set_pillar([x, z], new_water);
                }
            }

            if i[0] < chunks.xs.end - 1 {
                let mut left_results = [0u64; 64];
                let mut current_results = [0u64; 64];
                {
                    let left_ground = &chunks.ground[&[i[0] + 1, i[1]]];
                    let left_water = &new_water[&[i[0] + 1, i[1]]];
                    let current_water = &new_water[i];
                    for z in 0..64u8 {
                        let left_ground_column = left_ground.get_pillar([0, z]);
                        let left_water_column = left_water.get_pillar([0, z]);
                        let left_free = (!left_ground_column) & (!left_water_column);
                        let current_water = current_water.get_pillar([63, z]);
                        let water_flow = left_free & current_water;
                        let left_new_water = left_water_column | water_flow;
                        let new_water = current_water & (!water_flow);
                        left_results[z as usize] = left_new_water;
                        current_results[z as usize] = new_water;
                    }
                }
                {
                    let left_water = new_water.get_mut(&[i[0] + 1, i[1]]).unwrap();
                    for (i, v) in left_results.into_iter().enumerate() {
                        left_water.set_pillar([0, i as u8], v);
                    }
                }
                {
                    let current_water = new_water.get_mut(i).unwrap();
                    for (i, v) in current_results.into_iter().enumerate() {
                        current_water.set_pillar([63, i as u8], v);
                    }
                }
            }
        }
    } else if step_counter < 6 {
        for (i, ground) in chunks.ground.iter() {
            let water = new_water.get_mut(i).unwrap();
            for x in 0..64 {
                for z in 1..64 {
                    let left_ground_column = ground.get_pillar([x, z - 1]);
                    let left_water_column = water.get_pillar([x, z - 1]);
                    let left_free = (!left_ground_column) & (!left_water_column);
                    let current_water = water.get_pillar([x, z]);
                    let water_flow = left_free & current_water;
                    let left_new_water = left_water_column | water_flow;
                    let new_water = current_water & (!water_flow);
                    water.set_pillar([x, z - 1], left_new_water);
                    water.set_pillar([x, z], new_water);
                }
            }
            if i[1] > chunks.zs.start {
                let mut left_results = [0u64; 64];
                let mut current_results = [0u64; 64];
                {
                    let left_ground = &chunks.ground[&[i[0], i[1] - 1]];
                    let left_water = &new_water[&[i[0], i[1] - 1]];
                    let current_water = &new_water[i];
                    for x in 0..64u8 {
                        let left_ground_column = left_ground.get_pillar([x, 63]);
                        let left_water_column = left_water.get_pillar([x, 63]);
                        let left_free = (!left_ground_column) & (!left_water_column);
                        let current_water = current_water.get_pillar([x, 0]);
                        let water_flow = left_free & current_water;
                        let left_new_water = left_water_column | water_flow;
                        let new_water = current_water & (!water_flow);
                        left_results[x as usize] = left_new_water;
                        current_results[x as usize] = new_water;
                    }
                }
                {
                    let left_water = new_water.get_mut(&[i[0], i[1] - 1]).unwrap();
                    for (i, v) in left_results.into_iter().enumerate() {
                        left_water.set_pillar([i as u8, 63], v);
                    }
                }
                {
                    let current_water = new_water.get_mut(i).unwrap();
                    for (i, v) in current_results.into_iter().enumerate() {
                        current_water.set_pillar([i as u8, 0], v);
                    }
                }
            }
        }
    } else {
        for (i, ground) in chunks.ground.iter() {
            let water = new_water.get_mut(i).unwrap();
            for x in 0..64 {
                for z in 0..63 {
                    let left_ground_column = ground.get_pillar([x + 1, z]);
                    let left_water_column = water.get_pillar([x + 1, z]);
                    let left_free = (!left_ground_column) & (!left_water_column);
                    let current_water = water.get_pillar([x, z]);
                    let water_flow = left_free & current_water;
                    let left_new_water = left_water_column | water_flow;
                    let new_water = current_water & (!water_flow);
                    water.set_pillar([x + 1, z], left_new_water);
                    water.set_pillar([x, z], new_water);
                }
            }
            if i[1] < chunks.zs.end - 1 {
                let mut left_results = [0u64; 64];
                let mut current_results = [0u64; 64];
                {
                    let left_ground = &chunks.ground[&[i[0], i[1] + 1]];
                    let left_water = &new_water[&[i[0], i[1] + 1]];
                    let current_water = &new_water[i];
                    for x in 0..64u8 {
                        let left_ground_column = left_ground.get_pillar([x, 0]);
                        let left_water_column = left_water.get_pillar([x, 0]);
                        let left_free = (!left_ground_column) & (!left_water_column);
                        let current_water = current_water.get_pillar([x, 63]);
                        let water_flow = left_free & current_water;
                        let left_new_water = left_water_column | water_flow;
                        let new_water = current_water & (!water_flow);
                        left_results[x as usize] = left_new_water;
                        current_results[x as usize] = new_water;
                    }
                }
                {
                    let left_water = new_water.get_mut(&[i[0], i[1] + 1]).unwrap();
                    for (i, v) in left_results.into_iter().enumerate() {
                        left_water.set_pillar([i as u8, 0], v);
                    }
                }
                {
                    let current_water = new_water.get_mut(i).unwrap();
                    for (i, v) in current_results.into_iter().enumerate() {
                        current_water.set_pillar([i as u8, 63], v);
                    }
                }
            }
        }
    }
    chunks.water = new_water;
}

#[cfg(test)]
mod test {
    use crate::voxel_storage::VoxelWorld;

    use super::simulate_water;

    #[test]
    fn water_amount_stays_constant() {
        let mut world = VoxelWorld::gen(-2..2, -2..2);
        let total_water: u64 = world.water.values().map(|c| c.count()).sum();

        for i in 0..64 {
            simulate_water(&mut world, i);
            let water_after: u64 = world.water.values().map(|c| c.count()).sum();
            dbg!(i);
            assert_eq!(total_water, water_after);
        }
    }
}
