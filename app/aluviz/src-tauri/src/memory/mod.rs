mod tree;

use std::fmt::Display;

use crate::memory::tree::ByteSegmentTree;

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

// This interface defines all the operations possible on a memory model data type.
pub trait Memory {
    // This method should return the total capacity the model can represent in kb
    fn capacity(&self) -> usize;
    fn get_cell_width(&self) -> usize;
    fn mem_alloc(&mut self, elems: Vec<u8>) -> std::result::Result<usize, OutOfBoundsError>;
    fn loc(&self, pos: usize) -> u8;
}

// Represent single user memory schema. This means that memory is one continguous blob and its
// addresses starts from 0.
#[derive(Debug)]
pub struct SingleSchemeMemory {
    pub cell_size: usize,
    rows: usize,
    tree: tree::ByteSegmentTree,
}

impl SingleSchemeMemory {
    pub fn new((cell_size, rows): (usize, usize)) -> Self {
        SingleSchemeMemory {
            cell_size,
            rows,
            tree: ByteSegmentTree::new(rows),
        }
    }
}

impl Memory for SingleSchemeMemory {
    fn capacity(&self) -> usize {
        self.cell_size * self.rows
    }

    fn get_cell_width(&self) -> usize {
        self.cell_size
    }

    fn mem_alloc(&mut self, elems: Vec<u8>) -> std::result::Result<usize, OutOfBoundsError> {
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

// Represents memory with an additional abstraction of partitioning.
// Can reason about memory in terms of bins.
#[derive(Debug)]
pub struct FixedPartitionMemory {
    memory: SingleSchemeMemory,
    pub bin_count: usize,
    spread_factor: MemCustomizer,
}

impl FixedPartitionMemory {
    pub fn new(memory: SingleSchemeMemory) -> Self {
        FixedPartitionMemory {
            memory,
            bin_count: 1,
            spread_factor: MemCustomizer::DistributeBinsEvenly,
        }
    }

    pub fn allocate_bins(
        &mut self,
        num: usize,
        spread_factor: MemCustomizer,
    ) -> std::result::Result<(), OutOfBoundsError> {
        if num >= self.memory.rows {
            return Err(OutOfBoundsError);
        }
        self.bin_count = num;
        self.spread_factor = spread_factor;
        Ok(())
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

impl Memory for FixedPartitionMemory {
    fn capacity(&self) -> usize {
        self.memory.capacity()
    }

    fn get_cell_width(&self) -> usize {
        self.memory.cell_size
    }

    fn mem_alloc(&mut self, elems: Vec<u8>) -> std::result::Result<usize, OutOfBoundsError> {
        self.memory.mem_alloc(elems)
    }

    fn loc(&self, pos: usize) -> u8 {
        self.memory.loc(pos)
    }
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

impl MemCustomizer {
    pub fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
