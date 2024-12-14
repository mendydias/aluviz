use std::cmp::{max, min};

#[derive(Debug)]
pub struct ByteSegmentTree {
    start: usize,
    end: usize,
    store: Vec<usize>,
}

impl ByteSegmentTree {
    pub fn new(intervals: usize) -> Self {
        let store = build_tree_from(intervals);
        ByteSegmentTree {
            start: 0,
            end: intervals - 1,
            store,
        }
    }

    pub fn memsize(&self, root: usize, tl: usize, tr: usize, l: usize, r: usize) -> usize {
        if l > r {
            0
        } else if tl == l && tr == r {
            self.store[root]
        } else {
            let tm = (tl + tr) / 2;
            self.memsize(root * 2, tl, tm, l, min(r, tm))
                + self.memsize(root * 2 + 1, tm + 1, tr, max(l, tm + 1), r)
        }
    }
}

fn build_tree_from(intervals: usize) -> Vec<usize> {
    let mut store: Vec<usize> = vec![0; intervals * 4];
    build_tree_as_binheap(1, 0, intervals - 1, &mut store);
    store
}

fn build_tree_as_binheap(root: usize, tl: usize, tr: usize, t: &mut Vec<usize>) {
    if tl == tr {
        t[root] = 8;
    } else {
        let tm = (tl + tr) / 2;
        build_tree_as_binheap(root * 2, tl, tm, t);
        build_tree_as_binheap(root * 2 + 1, tm + 1, tr, t);
        t[root] = t[root * 2] + t[root * 2 + 1]
    }
}
