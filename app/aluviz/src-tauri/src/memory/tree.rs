use std::cmp::{max, min};

use super::OutOfBoundsError;

#[derive(Debug)]
pub struct ByteSegmentTree {
    start: usize,
    end: usize,
    store: Vec<Block>,
}

#[derive(Debug, Clone)]
enum Block {
    Empty,
    Full(BlockMetadata),
}

#[derive(Debug, Clone)]
struct BlockMetadata {
    address: usize,
    width: usize,
    contents: Vec<u8>,
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

    pub fn get(&self, v: usize, tl: usize, tr: usize, pos: usize) -> u8 {
        if tl == tr {
            return match &self.store[v] {
                Block::Full(d) => d.contents[0],
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

    fn update(
        &mut self,
        v: usize,
        tl: usize,
        tr: usize,
        pos: usize,
        val: u8,
    ) -> std::result::Result<usize, OutOfBoundsError> {
        if pos > tr {
            return Err(OutOfBoundsError);
        }
        if tl == tr {
            let new_block = match &self.store[v] {
                Block::Full(d) => BlockMetadata {
                    width: d.width,
                    address: d.address,
                    contents: vec![val],
                },
                Block::Empty => BlockMetadata {
                    width: 8,
                    address: tl,
                    contents: vec![val],
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

    fn build_tree_from(cells: usize) -> Vec<Block> {
        let mut store: Vec<Block> = vec![Block::Empty; cells * 4];
        Self::build_tree_as_binheap(1, 0, cells - 1, &mut store);
        store
    }

    fn build_tree_as_binheap(v: usize, tl: usize, tr: usize, t: &mut Vec<Block>) {
        if tl == tr {
            t[v] = Self::make_block(&t[v], 8, tl, vec![0]);
        } else {
            let tm = (tl + tr) / 2;
            Self::build_tree_as_binheap(v * 2, tl, tm, t);
            Self::build_tree_as_binheap(v * 2 + 1, tm + 1, tr, t);
            t[v] = Self::combine(&t[v * 2], &t[v * 2 + 1]);
        }
    }

    fn make_block(b: &Block, cell_width: usize, address: usize, elements: Vec<u8>) -> Block {
        match b {
            Block::Empty => Block::Full(BlockMetadata {
                address,
                width: cell_width,
                contents: elements,
            }),
            _ => panic!("Cannot overwrite full block."),
        }
    }

    fn combine(b1: &Block, b2: &Block) -> Block {
        match (b1, b2) {
            (Block::Full(d1), Block::Full(d2)) => {
                let mut contents: Vec<u8> = Vec::new();
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
                })
            }
            _ => panic!("Cannot combine empty blocks."),
        }
    }
}
