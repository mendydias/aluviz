mod utils;

use crate::memory::utils::ByteSegmentTree;

pub struct BasicMemory {
    cell_size: usize,
    rows: usize,
    bin_count: usize,
    spread_factor: MemCustomizer,
    memory: utils::ByteSegmentTree,
}

pub struct Bin {
    width: usize,
    address: u8,
}

pub enum MemCustomizer {
    DistributeBinsEvenly,
    // DistributeBinsRandomly,
}

pub trait CustomizeMemoryInit {
    fn default_mem_capacity() -> (usize, usize);
    //More methods to config from yaml config
}

impl BasicMemory {
    pub fn new((cell_size, rows): (usize, usize)) -> Self {
        let mut intervals: Vec<u8> = Vec::new();
        let rows8 = rows as u8;
        for i in 0..rows8 {
            intervals.push(i);
        }

        BasicMemory {
            cell_size,
            rows,
            bin_count: 0,
            spread_factor: MemCustomizer::DistributeBinsEvenly,
            memory: ByteSegmentTree::new(intervals),
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

    pub fn get_cell_width(&self) -> usize {
        self.cell_size
    }

    pub fn get_bins(&self) -> Vec<Bin> {
        match self.spread_factor {
            MemCustomizer::DistributeBinsEvenly => {
                let mut bins: Vec<Bin> = Vec::new();
                let b_width: u8 = self.rows as u8 / self.bin_count as u8;
                let mut start: u8 = 0;
                for i in 0..self.bin_count as u8 {
                    bins.push(Bin {
                        address: i * b_width,
                        width: (self.memory.memsize(
                            1,
                            0,
                            self.rows - 1,
                            start as usize,
                            (start + b_width) as usize,
                        ) * b_width) as usize,
                    });
                    start += b_width;
                }
                bins
            }
        }
    }
}

impl Bin {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_address(&self) -> u8 {
        self.address
    }
}

impl CustomizeMemoryInit for MemCustomizer {
    fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
