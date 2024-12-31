//! This module defines all memory structures and their behavior

mod errors;
mod tree;

use errors::{OutOfBoundsError, SimulatedMemoryResult};

use crate::memory::tree::ByteSegmentTree;

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

    /// Retrieves the entire memory data structure as it is currently in memory
    fn get_memory(&self) -> Vec<u8>;

    /// Lazily allocates over a block or a range of blocks
    fn range_alloc(&mut self, elems: Vec<u8>, l: usize) -> SimulatedMemoryResult<usize>;
}

/// Represent single user memory schema. This means that memory is one continguous blob and its
/// address always starts from 0.
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

    fn get_memory(&self) -> Vec<u8> {
        let (_, _, contents) = self
            .tree
            .get_root()
            .expect("Trying to access empty root node of tree");
        contents
    }

    fn range_alloc(&mut self, elems: Vec<u8>, l: usize) -> SimulatedMemoryResult<usize> {
        let data_size = elems.len();
        if data_size >= self.rows {
            return SimulatedMemoryResult::Err(OutOfBoundsError);
        }
        self.tree
            .allocate_over_range(&elems, 1, 0, self.rows - 1, l, data_size - 1);
        Ok(data_size)
    }
}

// Represents memory with an additional abstraction of partitioning.
// Can reason about memory in terms of bins.
#[derive(Debug)]
pub struct FixedPartitionMemory {
    memory: SingleSchemeMemory,
    pub bin_count: usize,
    bins: Vec<BinMetadata>,
    spread_factor: MemCustomizer,
}

impl FixedPartitionMemory {
    pub fn new(memory: SingleSchemeMemory) -> Self {
        FixedPartitionMemory {
            memory,
            bin_count: 1,
            bins: Vec::new(),
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
        self.update_bins();
        Ok(())
    }

    /// Convenience method to get the size of the bin in bytes.
    pub fn get_bin_width(&self) -> usize {
        self.memory.rows / self.bin_count * self.memory.cell_size
    }

    pub fn get_rows_per_bin(&self) -> usize {
        self.memory.rows / self.bin_count
    }

    /// Returns metadata about each individual bin
    pub fn get_bins(&self) -> &Vec<BinMetadata> {
        &self.bins
    }

    /// Private helper function that updates bin metadata.
    fn update_bins(&mut self) {
        self.bins = match self.spread_factor {
            MemCustomizer::DistributeBinsEvenly => {
                let mut bins: Vec<BinMetadata> = Vec::new();
                let b_width = self.memory.rows / self.bin_count;
                let mut start = 0;
                for i in 0..self.bin_count {
                    let address = i * b_width;
                    let l = start;
                    let r = start + b_width - 1;
                    let (width, free) =
                        self.memory
                            .tree
                            .get_mem_cell_state(1, 0, self.memory.rows - 1, l, r);
                    bins.push(BinMetadata {
                        address,
                        width,
                        free,
                    });
                    start += b_width;
                }
                bins
            }
        };
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

    fn get_memory(&self) -> Vec<u8> {
        self.memory.get_memory()
    }

    fn range_alloc(&mut self, elems: Vec<u8>, l: usize) -> SimulatedMemoryResult<usize> {
        let width = elems.len();
        let bin_width = self.get_rows_per_bin();
        if width >= bin_width {
            return SimulatedMemoryResult::Err(OutOfBoundsError);
        }
        self.memory.tree.allocate_over_range(
            &elems,
            1,
            0,
            self.memory.rows - 1,
            l,
            l + bin_width - 1,
        );
        Ok(l)
    }
}

/// Important data regarding bins.
/// These bins cannot be used to access bytes directly.
#[derive(Debug)]
pub struct BinMetadata {
    pub width: usize,
    pub address: usize,
    pub free: bool,
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
