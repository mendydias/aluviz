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

pub mod memory {

    pub struct BasicMemory {
        cell: usize,
        columns: usize,
    }

    pub struct MemCapacity {
        cell: usize,
        columns: usize,
    }

    pub enum Customizer {
        MemCustomizer,
    }

    pub trait MemCustomizer {
        fn default_mem_capacity() -> MemCapacity;
    }

    impl BasicMemory {
        pub fn new(mem_cap: MemCapacity) -> Self {
            let cell = mem_cap.cell;
            let columns = mem_cap.columns;
            BasicMemory { cell, columns }
        }

        pub fn cap(&self) -> usize {
            self.cell * self.columns
        }
    }

    impl MemCustomizer for Customizer {
        fn default_mem_capacity() -> MemCapacity {
            MemCapacity {
                cell: 8,
                columns: 64,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::memory::{BasicMemory, Customizer};

    #[test]
    fn test_memory_init() {
        let mem_customizer = Customizer::MemCustomizer();
        let mem = BasicMemory::new(Customizer::default_mem_capacity());
        let default_cap: usize = 64 * 8;
        assert_eq!(mem.cap(), default_cap);
    }

    #[test]
    fn test_memory_bin_setup() {
        let mem = memory::BasicMemory::new(memory::default_mem_capacity());
        let bin_count = 4;
        let bin_width = 128;
        mem.allocate_bins()
    }
}
