use crate::voxel_storage::{ChunkStorage, Chunks, VoxelStorage};

pub fn simulate_water(chunks: &mut Chunks) {
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

    chunks.water = new_water;
}
