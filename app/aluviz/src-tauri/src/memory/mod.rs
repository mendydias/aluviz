mod utils;

use crate::memory::utils::ByteSegmentTree;

#[derive(Debug)]
pub struct BasicMemory {
    cell_size: usize,
    rows: usize,
    bin_count: usize,
    spread_factor: MemCustomizer,
    memory: utils::ByteSegmentTree,
}

#[derive(Debug)]
pub struct Bin {
    width: usize,
    address: usize,
}

#[derive(Debug)]
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
        BasicMemory {
            cell_size,
            rows,
            bin_count: 0,
            spread_factor: MemCustomizer::DistributeBinsEvenly,
            memory: ByteSegmentTree::new(rows),
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
                let b_width = self.rows / self.bin_count;
                let mut start = 0;
                for i in 0..self.bin_count {
                    let address = i * b_width;
                    let l = start;
                    let r = start + b_width - 1;
                    let width = self.memory.memsize(1, 0, self.rows - 1, l, r);
                    bins.push(Bin { address, width });
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

    pub fn get_address(&self) -> usize {
        self.address
    }
}

impl CustomizeMemoryInit for MemCustomizer {
    fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
