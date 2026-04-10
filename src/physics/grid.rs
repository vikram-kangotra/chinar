use std::collections::HashMap;

use crate::math::vec3::Vec3;

pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn cell_coord(&self, pos: Vec3) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, index: usize, pos: Vec3) {
        let cell = self.cell_coord(pos);
        self.cells.entry(cell).or_insert_with(Vec::new).push(index);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn neighbors(&self, pos: Vec3, result: &mut Vec<usize>) {
        result.clear();

        let (cx, cy, cz) = self.cell_coord(pos);

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let key = (cx + dx, cy + dy, cz + dz);

                    if let Some(list) = self.cells.get(&key) {
                        result.extend(list);
                    }
                }
            }
        }
    }
}
