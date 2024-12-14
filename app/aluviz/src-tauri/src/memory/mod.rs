mod utils;

use crate::memory::utils::ByteSegmentTree;

pub struct BasicMemory {
    cell_size: usize,
    rows: usize,
    bin_count: usize,
    spread_factor: MemCustomizer,
    bins: utils::ByteSegmentTree,
}

pub enum MemCustomizer {
    DistributeBinsEvenly,
    // DistributeBinsRandomly,
}

pub trait CustomizeMemoryInit {
    fn default_mem_capacity() -> (usize, usize);
}

impl BasicMemory {
    pub fn new((cell_size, rows): (usize, usize)) -> Self {
        let mut intervals: Vec<u8> = Vec::new();
        let cell8 = cell_size as u8;
        let rows8 = rows as u8;
        for i in 0..rows8 {
            intervals.push(0 + cell8 * i);
        }

        BasicMemory {
            cell_size,
            rows,
            bin_count: 0,
            spread_factor: MemCustomizer::DistributeBinsEvenly,
            bins: ByteSegmentTree::new(intervals),
        }
    }

    pub fn cap(&self) -> usize {
        self.cell_size * self.rows
    }

    pub fn allocate_bins(&mut self, num: usize, spread_factor: MemCustomizer) {
        self.bin_count = num;
        self.spread_factor = spread_factor;
    }

    pub fn get_bin_count(&self) -> usize {
        self.bin_count
    }

    pub fn get_bin_width(&self) -> usize {
        self.rows / self.bin_count * self.cell_size
    }
}

impl CustomizeMemoryInit for MemCustomizer {
    fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
