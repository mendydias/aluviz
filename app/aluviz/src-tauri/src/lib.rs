// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod memory;

#[cfg(test)]
mod tests {
    use std::{fs::File, sync::Once};

    use simplelog::{Config, LevelFilter, WriteLogger};

    use crate::memory::{
        BasicMemory, CustomizeMemoryInit, MemCustomizer, Memory, PartitionedMemory,
    };

    static INIT_LOGGER_ONCE: Once = Once::new();

    fn setup_basic_mem() -> BasicMemory {
        BasicMemory::new(MemCustomizer::default_mem_capacity())
    }

    fn setup_partitioned_memory(basic_mem: BasicMemory) -> PartitionedMemory {
        PartitionedMemory::new(basic_mem)
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

    #[test]
    fn test_memory_init() {
        init_log();
        let mem = setup_basic_mem();
        let default_cap: usize = 32 * 8;
        assert_eq!(mem.capacity(), default_cap);
    }

    #[test]
    fn test_memory_bin_setup() {
        init_log();
        let mem = setup_basic_mem();
        let mut mem = setup_partitioned_memory(mem);
        let bin_count = 4;
        let bin_width = 64;
        mem.allocate_bins(4, MemCustomizer::DistributeBinsEvenly);
        // test bin count
        assert_eq!(mem.bin_count, bin_count);
        // test bin interval capacity
        assert_eq!(mem.get_bin_width(), bin_width);
    }

    #[test]
    fn test_get_bins() {
        init_log();
        let mem = setup_basic_mem();
        let mut mem = setup_partitioned_memory(mem);
        let bin_count = 4;
        let bin_width = 64;
        mem.allocate_bins(bin_count, MemCustomizer::DistributeBinsEvenly);
        let bins = mem.get_bins();
        assert_eq!(bins.len(), bin_count);
        for (i, bin) in bins.iter().enumerate() {
            assert_eq!(bin.width, bin_width);
            assert_eq!(bin.address, mem.get_cell_width() * i);
        }
    }

    #[test]
    fn test_allocate_cells() {
        init_log();
        let mut mem = setup_basic_mem();
        let value: u8 = 10;
        mem.mem_alloc(vec![value]);
        let alloc_address: usize = 0;
        let alloc_width: usize = 8;
        let first_bin = mem.get_bins()[0];
        let stored_value = first_bin.loc(alloc_address);
        assert_eq!(first_bin.get_address(), alloc_address);
        assert_eq!(stored_value, value);
        assert_eq!(first_bin.get_width(), alloc_width);
    }
}
