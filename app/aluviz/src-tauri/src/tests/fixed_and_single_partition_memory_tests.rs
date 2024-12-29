#[cfg(test)]
use crate::{
    memory::{FixedPartitionMemory, MemCustomizer, Memory, OutOfBoundsError},
    tests::{init_log, setup_single_scheme_mem},
};

#[test]
fn test_memory_init() {
    init_log();
    let mem = setup_single_scheme_mem();
    let mem = FixedPartitionMemory::new(mem);
    let default_cap: usize = 32 * 8;
    assert_eq!(mem.capacity(), default_cap);
}

#[test]
fn test_memory_bin_setup() -> Result<(), OutOfBoundsError> {
    init_log();
    let mem = setup_single_scheme_mem();
    let mut mem = FixedPartitionMemory::new(mem);
    let bin_count = 4;
    let bin_width = 64;
    let result = mem.allocate_bins(4, MemCustomizer::DistributeBinsEvenly);
    // test bin count
    assert_eq!(mem.bin_count, bin_count);
    // test bin interval capacity
    assert_eq!(mem.get_bin_width(), bin_width);
    result
}

#[test]
fn test_get_bins() -> Result<(), OutOfBoundsError> {
    init_log();
    let mem = setup_single_scheme_mem();
    let mut mem = FixedPartitionMemory::new(mem);
    let bin_count = 4;
    let bin_width = 64;
    let result = mem.allocate_bins(bin_count, MemCustomizer::DistributeBinsEvenly);
    let bins = mem.get_bins();
    assert_eq!(bins.len(), bin_count);
    for (i, bin) in bins.iter().enumerate() {
        assert_eq!(bin.width, bin_width);
        assert_eq!(bin.address, mem.get_cell_width() * i);
    }
    result
}

#[test]
fn test_allocate_cells_default() {
    init_log();
    let mem = setup_single_scheme_mem();
    let value: u8 = 10;
    // test if mem_alloc is successful
    let mut mem = FixedPartitionMemory::new(mem);
    let result = mem.mem_alloc(vec![value]);
    assert!(result.is_ok());
    let alloc_address: usize = 0;
    let stored_value = mem.loc(alloc_address);
    assert_eq!(stored_value, value);
}

#[test]
fn test_default_bin_allocation() {
    init_log();
    let mem = setup_single_scheme_mem();
    let mem = FixedPartitionMemory::new(mem);
    let alloc_address = 0;
    let alloc_width = mem.capacity();
    let bins = mem.get_bins();
    assert_eq!(bins.len(), 1);
    assert_eq!(bins[0].address, alloc_address);
    assert_eq!(bins[0].width, alloc_width);
}

#[test]
fn test_allocate_more_bins_than_rows() -> Result<(), OutOfBoundsError> {
    init_log();
    let mem = setup_single_scheme_mem();
    let mut mem = FixedPartitionMemory::new(mem);
    let result = mem.allocate_bins(40, MemCustomizer::DistributeBinsEvenly);
    if result.is_ok() {
        Err(OutOfBoundsError)
    } else {
        Ok(())
    }
}
