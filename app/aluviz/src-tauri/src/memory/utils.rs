pub struct ByteSegmentTree {
    store: Vec<u8>,
}

impl ByteSegmentTree {
    pub fn new(intervals: Vec<u8>) -> Self {
        let store = build_tree_from(intervals);
        ByteSegmentTree { store }
    }
}

fn build_tree_from(intervals: Vec<u8>) -> Vec<u8> {
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
        t[root] = t[root * 2] + (t[root * 2 + 1] - t[root * 2]);
    }
}
