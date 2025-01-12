use crate::memory::{self, Memory};

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

// Represents a queue of jobs that can be fed into a execution pipeline.
pub type JobQueue = Vec<Job>;

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
        // If there is no more free memory, we add it to the waiting jobs list
        for job in job_queue {
            // Check if the next job fits in the next available memory slot
            // TODO: maintain a list of free space pointers or one single pointer to the next free
            // bin.
        }
    }

    pub fn get_completed_jobs(&self) -> &JobQueue {
        &self.completed_jobs
    }
}

pub struct MemorySnapshot {
    current_memory: Vec<u8>,
}
