use crate::memory::{self, Memory};

pub enum AllocationPolicy {
    FirstFit,
}

pub struct MemoryManager {
    memory: Box<dyn Memory>,
    alloc_policy: AllocationPolicy,
}

impl MemoryManager {
    pub fn new(memory: impl Memory + 'static, alloc_policy: AllocationPolicy) -> Self {
        MemoryManager {
            memory: Box::new(memory),
            alloc_policy,
        }
    }

    pub fn get_memory(&self) -> &Box<dyn Memory> {
        &self.memory
    }
}
