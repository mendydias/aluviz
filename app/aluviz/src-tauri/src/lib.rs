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

    use log::debug;

    use simplelog::{Config, LevelFilter, WriteLogger};

    use crate::memory::{BasicMemory, CustomizeMemoryInit, MemCustomizer};

    static INIT_LOGGER_ONCE: Once = Once::new();

    fn setup_mem() -> BasicMemory {
        BasicMemory::new(MemCustomizer::default_mem_capacity())
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
        let mem = setup_mem();
        debug!("Current memory model inside test_memory_init: {:?}", mem);
        let default_cap: usize = 32 * 8;
        assert_eq!(mem.cap(), default_cap);
    }

    #[test]
    fn test_memory_bin_setup() {
        let mut mem = setup_mem();
        let bin_count = 4;
        let bin_width = 64;
        mem.allocate_bins(4, MemCustomizer::DistributeBinsEvenly);
        debug!(
            "Current memory model inside test_memory_bin_setup: {:?}",
            mem
        );
        // test bin count
        assert_eq!(mem.get_bin_count(), bin_count);
        // test bin interval capacity
        assert_eq!(mem.get_bin_width(), bin_width);
    }

    #[test]
    fn test_get_bins() {
        let mut mem = setup_mem();
        let bin_count = 4;
        let bin_width = 64;
        mem.allocate_bins(bin_count, MemCustomizer::DistributeBinsEvenly);
        debug!("Current memory model insdie test_get_bins: {:?}", mem);
        let bins = mem.get_bins();
        assert_eq!(bins.len(), bin_count);
        for (i, bin) in bins.iter().enumerate() {
            assert_eq!(bin.get_width(), bin_width);
            assert_eq!(bin.get_address(), mem.get_cell_width() * i);
        }
    }

    #[test]
    fn test_allocate_cells() {
        todo!();
    }
}
