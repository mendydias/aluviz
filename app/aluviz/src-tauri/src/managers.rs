use crate::memory::{self, Memory};

pub enum AllocationPolicy {
    FirstFit,
}

pub struct MemoryManager {
    memory: Box<dyn Memory>,
    alloc_policy: AllocationPolicy,
    completed_jobs: JobQueue,
    waiting_jobs: JobQueue,
}

impl MemoryManager {
    pub fn new(memory: impl Memory + 'static, alloc_policy: AllocationPolicy) -> Self {
        MemoryManager {
            memory: Box::new(memory),
            alloc_policy,
            completed_jobs: JobQueue::new(),
            waiting_jobs: JobQueue::new(),
        }
    }

    pub fn get_memory(&self) -> &Box<dyn Memory> {
        &self.memory
    }

    pub fn get_curr_mem_snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            current_memory: self.memory.get_memory(),
        }
    }

    pub fn load_memory(&self, job_queue: JobQueue) {
        // Take each job in the queue and load it into memory.
        // If there is no more free memory, we add it to the waitign jobs list
        for 
    }
}

pub struct MemorySnapshot {
    current_memory: Vec<u8>,
}

pub struct Job {
    arrival: usize,
    burst: usize,
    size: usize,
}

impl Job {
    pub fn new(size: usize, burst: usize, arrival: usize) -> Self {
        Job {
            size,
            burst,
            arrival,
        }
    }
}

pub struct JobQueue {
    pub jobs: Vec<Job>,
}

impl JobQueue {
    pub fn init_from(jobs: Vec<Job>) -> Self {
        JobQueue { jobs }
    }

    pub fn new() -> Self {
        JobQueue { jobs: Vec::new() }
    }
}
