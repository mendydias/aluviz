mod utils;

use std::fmt::Display;

use serde::de::Error;

use crate::memory::utils::ByteSegmentTree;

// Common error type to gracefully handle out of bounds errors
pub type Result<T> = std::result::Result<T, OutOfBoundsError>;

#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request memory range is out of bounds for the simulation"
        )
    }
}

// The most basic type of memory. All it does is represent memory as a contiguous block of memory
// starting from address 0. Memory allocated to this model will write elements sequentially
// starting from address 0.
#[derive(Debug)]
pub struct BasicMemory {
    pub cell_size: usize,
    rows: usize,
    tree: utils::ByteSegmentTree,
}

// Represents memory with an additional abstraction of partitioning.
// Can reason about memory in terms of bins.
#[derive(Debug)]
pub struct PartitionedMemory {
    memory: BasicMemory,
    pub bin_count: usize,
    spread_factor: MemCustomizer,
}

#[derive(Debug)]
pub struct Bin {
    pub width: usize,
    pub address: usize,
}

#[derive(Debug)]
pub enum MemCustomizer {
    DistributeBinsEvenly,
    // DistributeBinsRandomly,
}

pub trait CustomizeMemoryInit {
    fn default_mem_capacity() -> (usize, usize);
    // TODO: More config methods through a yaml config
}

pub trait Memory {
    fn capacity(&self) -> usize;
    fn get_cell_width(&self) -> usize;
    fn mem_alloc(&mut self, elems: Vec<u8>) -> Result<usize>;
    fn loc(&self, pos: usize) -> u8;
}

impl BasicMemory {
    pub fn new((cell_size, rows): (usize, usize)) -> Self {
        BasicMemory {
            cell_size,
            rows,
            tree: ByteSegmentTree::new(rows),
        }
    }
}

impl Memory for BasicMemory {
    fn capacity(&self) -> usize {
        self.cell_size * self.rows
    }

    fn get_cell_width(&self) -> usize {
        self.cell_size
    }

    fn mem_alloc(&mut self, elems: Vec<u8>) -> Result<usize> {
        if elems.len() < self.rows {
            let updated: usize = self.tree.update_from(elems, 0);
            Ok(updated * self.cell_size)
        } else {
            Result::Err(OutOfBoundsError)
        }
    }

    fn loc(&self, pos: usize) -> u8 {
        self.tree.get(1, 0, self.rows - 1, pos)
    }
}

impl PartitionedMemory {
    pub fn new(memory: BasicMemory) -> Self {
        PartitionedMemory {
            memory,
            bin_count: 1,
            spread_factor: MemCustomizer::DistributeBinsEvenly,
        }
    }

    pub fn allocate_bins(&mut self, num: usize, spread_factor: MemCustomizer) {
        self.bin_count = num;
        self.spread_factor = spread_factor;
    }

    pub fn get_bin_width(&self) -> usize {
        self.memory.rows / self.bin_count * self.memory.cell_size
    }

    pub fn get_bins(&self) -> Vec<Bin> {
        match self.spread_factor {
            MemCustomizer::DistributeBinsEvenly => {
                let mut bins: Vec<Bin> = Vec::new();
                let b_width = self.memory.rows / self.bin_count;
                let mut start = 0;
                for i in 0..self.bin_count {
                    let address = i * b_width;
                    let l = start;
                    let r = start + b_width - 1;
                    let width = self.memory.tree.memsize(1, 0, self.memory.rows - 1, l, r);
                    bins.push(Bin { address, width });
                    start += b_width;
                }
                bins
            }
        }
    }
}

impl Memory for PartitionedMemory {
    fn capacity(&self) -> usize {
        self.memory.capacity()
    }

    fn get_cell_width(&self) -> usize {
        self.memory.cell_size
    }

    fn mem_alloc(&mut self, elems: Vec<u8>) -> Result<usize> {
        self.memory.mem_alloc(elems)
    }

    fn loc(&self, pos: usize) -> u8 {
        self.memory.loc(pos)
    }
}

impl CustomizeMemoryInit for MemCustomizer {
    fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
