pub enum ProcessorCore {
    SingleUnitSingleThreaded,
}

pub enum Pipeline {
    SingleSequential,
}

pub enum ClockSpeed {
    None,
}

pub struct Processor {
    core_setup: ProcessorCore,
    exec_pipeline: Pipeline,
    pub clockspeed: ClockSpeed,
}

impl Processor {
    pub fn new(core_setup: ProcessorCore, exec_pipeline: Pipeline, clockspeed: ClockSpeed) -> Self {
        Processor {
            core_setup,
            exec_pipeline,
            clockspeed,
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
