pub const WORLD_SIZE: usize = 40;
pub const MAX_HEIGHT: u8 = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind { Grass, Dirt, Stone, Sand, Wood }

#[derive(Clone, Debug)]
pub struct World {
    heights: Vec<u8>,
    tops: Vec<BlockKind>,
    pub seed: u32,
}

impl World {
    pub fn new(seed: u32) -> Self {
        let mut heights = vec![0; WORLD_SIZE * WORLD_SIZE];
        let mut tops = vec![BlockKind::Grass; WORLD_SIZE * WORLD_SIZE];
        for z in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let xf = x as f32;
                let zf = z as f32;
                let broad = (xf * 0.19).sin() * 1.55
                    + (zf * 0.16).cos() * 1.45
                    + ((xf + zf) * 0.11).sin() * 0.9;
                let detail = (hash(seed, x as u32, z as u32) % 100) as f32 / 100.0;
                let height = (5.0 + broad + (detail - 0.5) * 1.4).round().clamp(3.0, 10.0) as u8;
                let index = Self::index(x, z);
                heights[index] = height;
                if height <= 4 { tops[index] = BlockKind::Sand; }
            }
        }
        Self { heights, tops, seed }
    }

    fn index(x: usize, z: usize) -> usize { z * WORLD_SIZE + x }

    pub fn height_at(&self, x: i32, z: i32) -> Option<u8> {
        if x < 0 || z < 0 || x >= WORLD_SIZE as i32 || z >= WORLD_SIZE as i32 { return None; }
        Some(self.heights[Self::index(x as usize, z as usize)])
    }

    pub fn top_at(&self, x: i32, z: i32) -> Option<BlockKind> {
        if x < 0 || z < 0 || x >= WORLD_SIZE as i32 || z >= WORLD_SIZE as i32 { return None; }
        Some(self.tops[Self::index(x as usize, z as usize)])
    }

    pub fn mine_top(&mut self, x: i32, z: i32) -> bool {
        let Some(height) = self.height_at(x, z) else { return false; };
        if height <= 1 { return false; }
        let index = Self::index(x as usize, z as usize);
        self.heights[index] -= 1;
        self.tops[index] = match self.tops[index] {
            BlockKind::Grass => BlockKind::Dirt,
            BlockKind::Dirt => BlockKind::Stone,
            other => other,
        };
        true
    }

    pub fn place_top(&mut self, x: i32, z: i32, kind: BlockKind) -> bool {
        let Some(height) = self.height_at(x, z) else { return false; };
        if height >= MAX_HEIGHT { return false; }
        let index = Self::index(x as usize, z as usize);
        self.heights[index] += 1;
        self.tops[index] = kind;
        true
    }

    pub fn has_tree(&self, x: i32, z: i32) -> bool {
        if x < 3 || z < 3 || x >= WORLD_SIZE as i32 - 3 || z >= WORLD_SIZE as i32 - 3 { return false; }
        if (x - WORLD_SIZE as i32 / 2).abs() < 3 && (z - WORLD_SIZE as i32 / 2).abs() < 3 { return false; }
        hash(self.seed ^ 0x9E37_79B9, x as u32, z as u32) % 59 == 0
    }
}

fn hash(seed: u32, x: u32, z: u32) -> u32 {
    let mut value = seed ^ x.wrapping_mul(0x045D_9F3B) ^ z.wrapping_mul(0x119D_E1F3);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^ (value >> 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_world_is_repeatable_and_bounded() {
        let a = World::new(42);
        let b = World::new(42);
        for z in 0..WORLD_SIZE as i32 {
            for x in 0..WORLD_SIZE as i32 {
                assert_eq!(a.height_at(x, z), b.height_at(x, z));
                assert!((3..=10).contains(&a.height_at(x, z).unwrap()));
            }
        }
        assert_eq!(a.height_at(-1, 0), None);
        assert_eq!(a.height_at(WORLD_SIZE as i32, 0), None);
    }

    #[test]
    fn block_mining_and_placing_work() {
        let mut world = World::new(7);
        let before = world.height_at(20, 20).unwrap();
        assert!(world.mine_top(20, 20));
        assert_eq!(world.height_at(20, 20), Some(before - 1));
        assert!(world.place_top(20, 20, BlockKind::Stone));
        assert_eq!(world.height_at(20, 20), Some(before));
        assert_eq!(world.top_at(20, 20), Some(BlockKind::Stone));
    }

    #[test]
    fn edits_outside_world_and_overflow_are_rejected() {
        let mut world = World::new(99);
        assert!(!world.mine_top(-1, 2));
        assert!(!world.place_top(WORLD_SIZE as i32, 2, BlockKind::Dirt));
        while world.height_at(10, 10).unwrap() < MAX_HEIGHT {
            assert!(world.place_top(10, 10, BlockKind::Dirt));
        }
        assert!(!world.place_top(10, 10, BlockKind::Dirt));
    }
}
