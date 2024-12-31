//! An implementation of a segment tree, which represents virtual memory.

use std::cmp::{max, min};

use super::OutOfBoundsError;

/// The main data structure that represents the memory model.
///
/// The idea is that the underlying structure of memory doesn't change, therefore most information
/// about the model can be pre-calculated and stored for faster access and rendering.
#[derive(Debug)]
pub struct ByteSegmentTree {
    start: usize,
    end: usize,
    store: Vec<Block>,
}

/// Blocks represent a unit of memory independent of the byte, e.g. spanning from one byte to
/// several.
///
/// In most simulation contexts, the block will represent a word.
#[derive(Debug, Clone)]
enum Block {
    Empty,
    Full(BlockMetadata),
}

type MemoryCell = Option<u8>;

/// While a block represents a unit of memory, it doesn't hold any data specific to the memory that
/// it maps. Instead, the following struct stores the actual data that is contained within that
/// memory segment.
#[derive(Debug, Clone)]
struct BlockMetadata {
    address: usize,
    width: usize,
    free: bool,
    dirty: bool,
    contents: Vec<MemoryCell>,
}

impl ByteSegmentTree {
    pub fn new(cells: usize) -> Self {
        let store = Self::build_tree_from(cells);
        ByteSegmentTree {
            start: 0,
            end: cells - 1,
            store,
        }
    }

    /// Traverses down the tree and calculates the width of the given range l to r.
    ///
    /// v: root vertex
    /// tl: the lower bound of the current range
    /// tr: the upper bound of the current range
    /// l: the lower bound of the range to calculate the width
    /// r: the upper bound fo the range to calculate the width
    pub fn memsize(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> usize {
        if l > r {
            0
        } else if tl == l && tr == r {
            match &self.store[v] {
                Block::Empty => 0,
                Block::Full(d) => d.width,
            }
        } else {
            let tm = (tl + tr) / 2;
            self.memsize(v * 2, tl, tm, l, min(r, tm))
                + self.memsize(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r)
        }
    }

    /// The following function returns the following information about a range of blocks:
    ///     - what is the width of the block?
    ///     - is the block allocated or free?
    pub fn get_mem_cell_state(
        &self,
        v: usize,  // Current root index
        tl: usize, // Left-most index of current sub-tree
        tr: usize, // Right-most index of current sub-tree
        l: usize,  // Starting index of range
        r: usize,  // Ending index of range
    ) -> (usize, bool) {
        if l > r || tl > tr {
            (0, false)
        } else if tl == l && tr == r {
            match &self.store[v] {
                Block::Empty => (0, false),
                Block::Full(d) => (d.width, d.free),
            }
        } else {
            let tm = (tl + tr) / 2;
            let left_child = self.get_mem_cell_state(v * 2, tl, tm, l, min(r, tm));
            let right_child = self.get_mem_cell_state(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r);
            (left_child.0 + right_child.0, left_child.1 && right_child.1)
        }
    }

    /// Given a position in the array, the function first traverses down the tree to find the
    /// position. Then, it overwrites the contents of memory with each of the elements given to the
    /// function.
    /// TODO: Change this method to exclusively handle single value updates
    pub fn update_from(&mut self, elems: Vec<u8>, pos: usize) -> usize {
        let mut mem_changed: usize = 0;
        for (index, item) in elems.iter().enumerate() {
            let result = self.update(1, self.start, self.end, pos + index, item.to_owned());
            if let Ok(width) = result {
                mem_changed += width;
            }
        }
        mem_changed
    }

    /// The tree is ordered by the leaf nodes which are in the same position as in their original
    /// array. This function uses binary search to retrieve the element in the current range given
    /// by the position.
    pub fn get(&self, v: usize, tl: usize, tr: usize, pos: usize) -> u8 {
        if tl == tr {
            return match &self.store[v] {
                Block::Full(d) => d.contents[0].unwrap_or(0),
                Block::Empty => 0,
            };
        }
        let tm = (tl + tr) / 2;
        if pos <= tm {
            self.get(v * 2, tl, tm, pos)
        } else {
            self.get(v * 2 + 1, tm + 1, tr, pos)
        }
    }

    /// This function retrieves an immutable reference to the root node of the current tree.
    pub fn get_root(&self) -> Option<(usize, usize, Vec<u8>)> {
        match &self.store[1] {
            Block::Empty => Option::None,
            Block::Full(d) => Option::Some((
                d.width,
                d.address,
                d.contents.iter().map(|cell| cell.unwrap_or(0)).collect(),
            )),
        }
    }

    /// Private helper function to update a value in the tree.
    /// Here pos and address are the same. Imagine the memory layout to be an array of bytes. Each
    /// pos and address refer to an index here.
    fn update(
        &mut self,
        v: usize,
        tl: usize,
        tr: usize,
        pos: usize,
        val: u8,
    ) -> std::result::Result<usize, OutOfBoundsError> {
        if pos > tr || pos < tl {
            // If the given position is beyond the range tl to tr, the function throws an out of
            // bounds error.
            return Err(OutOfBoundsError);
        }
        if tl == tr {
            let new_block = match &self.store[v] {
                Block::Full(d) => BlockMetadata {
                    width: d.width,
                    address: d.address,
                    contents: vec![MemoryCell::Some(val)],
                    free: false,
                    dirty: false,
                },
                Block::Empty => BlockMetadata {
                    width: 8,
                    address: tl,
                    contents: vec![MemoryCell::Some(val)],
                    free: false,
                    dirty: false,
                },
            };
            self.store[v] = Block::Full(new_block);
            Ok(8)
        } else {
            let tm = (tl + tr) / 2;
            let result = if pos <= tm {
                self.update(v * 2, tl, tm, pos, val)
            } else {
                self.update(v * 2 + 1, tm + 1, tr, pos, val)
            };
            Self::combine(&self.store[v * 2], &self.store[v * 2 + 1]);
            result
        }
    }

    /// Builds a segment tree with the given number of elements
    fn build_tree_from(cells: usize) -> Vec<Block> {
        let mut store: Vec<Block> = vec![Block::Empty; cells * 4];
        Self::build_tree_as_binheap(1, 0, cells - 1, &mut store);
        store
    }

    /// The main function that creates the binary tree in memory. This is only run once, when
    /// memory is initialized.
    fn build_tree_as_binheap(v: usize, tl: usize, tr: usize, t: &mut Vec<Block>) {
        if tl == tr {
            t[v] = Self::make_block(&t[v], 8, tl, vec![MemoryCell::None]);
        } else {
            let tm = (tl + tr) / 2;
            Self::build_tree_as_binheap(v * 2, tl, tm, t);
            Self::build_tree_as_binheap(v * 2 + 1, tm + 1, tr, t);
            t[v] = Self::combine(&t[v * 2], &t[v * 2 + 1]);
        }
    }

    /// Function to initialize the tree with blocks
    fn make_block(
        b: &Block,
        cell_width: usize,
        address: usize,
        elements: Vec<MemoryCell>,
    ) -> Block {
        match b {
            Block::Empty => Block::Full(BlockMetadata {
                address,
                width: cell_width,
                contents: elements,
                free: true,
                dirty: false,
            }),
            _ => panic!("Cannot overwrite full block."),
        }
    }

    /// Since blocks don't have the + operator defined, this method defines how two blocks should
    /// be combined for each parent node that pre-computes the width of the range.
    fn combine(b1: &Block, b2: &Block) -> Block {
        match (b1, b2) {
            (Block::Full(d1), Block::Full(d2)) => {
                let mut contents: Vec<MemoryCell> = Vec::new();
                for v in d1.contents.iter() {
                    contents.push(v.to_owned());
                }
                for v in d2.contents.iter() {
                    contents.push(v.to_owned());
                }

                Block::Full(BlockMetadata {
                    address: d1.address,
                    width: d1.width + d2.width,
                    contents,
                    free: d1.free && d2.free,
                    dirty: false,
                })
            }
            _ => panic!("Cannot combine empty blocks."),
        }
    }

    /// Allocates memory over a range of addresses using the given array of elements.
    pub fn allocate_over_range(
        &mut self,
        elems: &Vec<u8>, // Elements to add
        v: usize,        // Current root index
        tl: usize,       // Left-most index of sub-tree
        tr: usize,       // Right-most index of sub-tree
        l: usize,        // Starting index of memory range
        r: usize,        // Ending index of memory range
    ) {
        if l > r {
            return;
        } else if tl == l && tr == r {
            if let Block::Full(d) = &mut self.store[v] {
                d.contents = elems
                    .iter()
                    .map(|item| MemoryCell::Some(item.clone()))
                    .collect();
                d.dirty = true;
            }
        } else {
            self.push(v);
            let tm = (tl + tr) / 2;
            self.allocate_over_range(elems, v * 2, tl, tm, l, min(r, tm));
            self.allocate_over_range(elems, v * 2 + 1, tm + 1, tr, max(tm + 1, l), r);
        }
    }

    /// Helper function to propagate changes down the sub-tree.
    fn push(&mut self, index: usize) {
        let contents = match &mut self.store[index] {
            Block::Empty => panic!("Cannot push lazy operations down an empty node"),
            Block::Full(d) => {
                let is_dirty = d.dirty;
                d.dirty = false;
                (is_dirty, &d.contents)
            }
        };
        // If the contents are dirty, then push the operation down to the children.
        if contents.0 {
            let tm = contents.1.len() / 2;
            let left_child_contents = contents.1[..tm].to_vec();
            let right_child_contents = contents.1[tm + 1..].to_vec();
            // FIX: The following two calls might fail if they are uninitialized (a.k.a Empty).
            // Consider adding an escape `else` clause to handle the Block::Empty case.
            if let Block::Full(l) = &mut self.store[index * 2] {
                l.dirty = true;
                l.contents = left_child_contents;
            }
            if let Block::Full(r) = &mut self.store[index * 2 + 1] {
                r.dirty = true;
                r.contents = right_child_contents;
            }
        }
    }
}
