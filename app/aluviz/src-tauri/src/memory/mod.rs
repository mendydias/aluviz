mod errors;
mod tree;

use std::fmt::Display;

use crate::memory::tree::ByteSegmentTree;

/// OutOfBoundsError represents the cases where the user tries to allocate memory or configure bins
/// such that the configured memory exceeds the total memory.
#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request memory range is out of bounds for the simulation."
        )
    }
}

/// This interface defines all the operations possible on a memory model data type.
///
/// The memory model makes the following assumptions:
///
/// - The size of one cell is 8 bits or 1 byte
/// - Memory is laid out contiguously, like an array, with each cell followed by another.
/// - The memory model is byte addressible.
///
/// Therefore, the capacity of this memory model is the number of rows times the size of the cell
/// in bytes.
pub trait Memory {
    /// This method should return the total capacity the model can represent in bytes
    fn capacity(&self) -> usize;

    /// This is a convenience method to return the size of a single cell
    fn get_cell_width(&self) -> usize;

    /// Allocates memory for the given elements. Throws an OutOfBoundsError if the number of
    /// elements exceed the total capacity.
    fn mem_alloc(&mut self, elems: Vec<u8>) -> std::result::Result<usize, OutOfBoundsError>;

    /// Retrieves the byte at the given location.
    fn loc(&self, pos: usize) -> u8;
}

/// Represent single user memory schema. This means that memory is one continguous blob and its
/// addresses starts from 0.
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

    /// This method is what separates FixedPartitionMemory from SingleSchemeMemory. It creates an
    /// abstraction over a single blob of memory where addresses are grouped into bins. The spread
    /// factor decides how the bins are allocated.
    ///
    /// Throws an OutOfBoundsError if the requested number of bins exceed the smallest possible
    /// bin: the byte.
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

    /// Convenience method to get the size of the bin in bytes.
    pub fn get_bin_width(&self) -> usize {
        self.memory.rows / self.bin_count * self.memory.cell_size
    }

    /// Returns metadata about each individual bin
    pub fn get_bins(&self) -> Vec<BinMetadata> {
        match self.spread_factor {
            MemCustomizer::DistributeBinsEvenly => {
                let mut bins: Vec<BinMetadata> = Vec::new();
                let b_width = self.memory.rows / self.bin_count;
                let mut start = 0;
                for i in 0..self.bin_count {
                    let address = i * b_width;
                    let l = start;
                    let r = start + b_width - 1;
                    let width = self.memory.tree.memsize(1, 0, self.memory.rows - 1, l, r);
                    bins.push(BinMetadata { address, width });
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

/// Important data regarding bins.
/// These bins cannot be used to access bytes directly.
#[derive(Debug)]
pub struct BinMetadata {
    pub width: usize,
    pub address: usize,
}

/// This configuration setting is used to determine how the bins are distributed over the entirety
/// of memory.
#[derive(Debug)]
pub enum MemCustomizer {
    DistributeBinsEvenly,
    // DistributeBinsRandomly,
}

impl MemCustomizer {
    /// A 256 byte memory is the default model.
    pub fn default_mem_capacity() -> (usize, usize) {
        (8, 32)
    }
}
