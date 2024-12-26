use std::{
    fs::{self, File},
    path::Path,
    sync::Once,
};

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
        let path = "./logs";
        let path = Path::new(path);
        if !path.is_dir() {
            fs::create_dir(path).expect("cannot create log folder");
        }
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("./logs/tests.log").unwrap(), // TODO: test if the log
                                                       // folder exists and if not create the log folder and the log file.
        )
        .unwrap();
    });
}
