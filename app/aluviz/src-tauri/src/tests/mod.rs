use std::{fs::File, sync::Once};

use log::LevelFilter;
use simplelog::{Config, WriteLogger};

use crate::memory::{FixedPartitionMemory, MemCustomizer, SingleSchemeMemory};

#[cfg(test)]
mod fixed_and_single_partition_memory_tests;

static INIT_LOGGER_ONCE: Once = Once::new();

fn setup_basic_mem() -> SingleSchemeMemory {
    SingleSchemeMemory::new(MemCustomizer::default_mem_capacity())
}

fn setup_partitioned_memory(basic_mem: SingleSchemeMemory) -> FixedPartitionMemory {
    FixedPartitionMemory::new(basic_mem)
}

fn init_log() {
    INIT_LOGGER_ONCE.call_once(|| {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("./logs/tests.log").unwrap(),
        )
        .unwrap();
    });
}
