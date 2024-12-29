pub enum ProcessorCore {
    SingleUnitSingleThreaded,
}

pub enum Pipeline {
    SingleSequential,
}

pub struct Processor {
    core_setup: ProcessorCore,
    exec_pipeline: Pipeline,
}

impl Processor {
    pub fn new(core_setup: ProcessorCore, exec_pipeline: Pipeline) -> Self {
        Processor {
            core_setup,
            exec_pipeline,
        }
    }
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
    jobs: Vec<Job>,
}

impl JobQueue {
    pub fn init_from(jobs: Vec<Job>) -> Self {
        JobQueue { jobs }
    }
}
