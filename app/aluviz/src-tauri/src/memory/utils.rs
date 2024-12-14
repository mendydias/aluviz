use std::cmp::{max, min};

pub struct ByteSegmentTree {
    start: usize,
    end: usize,
    store: Vec<u8>,
}

impl ByteSegmentTree {
    pub fn new(intervals: Vec<u8>) -> Self {
        let store = build_tree_from(&intervals);
        ByteSegmentTree {
            start: 0,
            end: intervals.len() - 1,
            store,
        }
    }

    pub fn memsize(&self, root: usize, tl: usize, tr: usize, l: usize, r: usize) -> u8 {
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

fn build_tree_from(intervals: &Vec<u8>) -> Vec<u8> {
    let mut store = vec![0; intervals.len() * 4];
    build_tree_as_binheap(&intervals, 1, 0, intervals.len() - 1, &mut store);
    store
}

fn build_tree_as_binheap(a: &Vec<u8>, root: usize, tl: usize, tr: usize, t: &mut Vec<u8>) {
    if tl == tr {
        t[root] = a[tl];
    } else {
        let tm = (tl + tr) / 2;
        build_tree_as_binheap(a, root * 2, tl, tm, t);
        build_tree_as_binheap(a, root * 2 + 1, tm + 1, tr, t);
        t[root] = t[root * 2] + t[root * 2 + 1];
    }
}
