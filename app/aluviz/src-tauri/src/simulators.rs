use crate::{
    managers::{JobQueue, MemoryManager, MemorySnapshot},
    processors::Processor,
};

pub enum Lifetime {
    Specific(u32),
}

pub struct Simulation {
    memory_manager: MemoryManager,
    processor: Processor,
    job_queue: JobQueue,
    current_frame: Option<Frame>,
}

impl Simulation {
    pub fn new(memory_manager: MemoryManager, processor: Processor, job_queue: JobQueue) -> Self {
        Simulation {
            memory_manager,
            processor,
            job_queue,
            current_frame: Option::None,
        }
    }
}

pub struct Frame {
    memory_snapshot: MemorySnapshot,
    completed: &JobQueue,
    waiting: &JobQueue,
    unallocated_jobs: JobQueue,
}

pub trait Simulator {
    fn play_to_end(&self) -> Frame;
}

impl Simulator for Simulation {
    fn play_to_end(&self) -> Frame {
        match self.current_frame {
            _ => {
                self.memory_manager.load_memory(self.job_queue);
                self.processor.execute(&self.memory_manager);
                return Frame {
                    completed: self.memory_manager.get_completed_jobs(),
                    waiting: self.memory_manager.get_waiting_queue(),
                    memory_snapshot: self.memory_manager.get_curr_mem_snapshot(),
                    unallocated_jobs: self.memory_manager.get_unallocated_jobs(),
                };
            }
        }
    }
}
