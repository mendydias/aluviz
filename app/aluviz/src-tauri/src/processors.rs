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
