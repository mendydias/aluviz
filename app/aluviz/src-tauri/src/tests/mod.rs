use std::{
    fs::{self, File},
    path::Path,
    sync::Once,
};

use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::{
    managers::{AllocationPolicy, MemoryManager},
    memory::Memory,
};
use crate::{
    memory::{FixedPartitionMemory, MemCustomizer, SingleSchemeMemory},
    processors::{Pipeline, Processor, ProcessorCore},
};

static INIT_LOGGER_ONCE: Once = Once::new();

fn setup_single_scheme_mem() -> SingleSchemeMemory {
    SingleSchemeMemory::new(MemCustomizer::default_mem_capacity())
}

fn init_log() {
    // We set up logger for this once and if for some reason, I call set_logger elsewhere, it will
    // just ignore setting up this logger instance.
    INIT_LOGGER_ONCE.call_once(|| {
        TermLogger::init(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .unwrap();
    });
}

fn setup_memory_manager(
    memory: impl Memory + 'static,
    alloc_policy: AllocationPolicy,
) -> MemoryManager {
    MemoryManager::new(memory, alloc_policy)
}

fn setup_basic_inout_processor(core_setup: ProcessorCore, exec_pipeline: Pipeline) -> Processor {
    Processor::new(core_setup, exec_pipeline)
}

fn setup_basic_first_fit_components() -> (MemoryManager, Processor) {
    let mem = setup_single_scheme_mem();
    let manager = setup_memory_manager(mem, AllocationPolicy::FirstFit);
    let processor = setup_basic_inout_processor(
        ProcessorCore::SingleUnitSingleThreaded,
        Pipeline::SingleSequential,
    );
    (manager, processor)
}

mod fixed_and_single_partition_memory_tests;
mod memory_allocation_algorithm_tests;
