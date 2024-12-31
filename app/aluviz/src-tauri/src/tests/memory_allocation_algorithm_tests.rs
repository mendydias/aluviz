use super::setup_basic_first_fit_components;
use crate::{
    managers::{Job, JobQueue},
    simulators::{Simulation, Simulator},
};

#[test]
fn test_first_fit_single_scheme() {
    let (manager, processor) = setup_basic_first_fit_components();
    // define some jobs for the simulation
    let job1 = Job::new(32, 4, 0);
    let job2 = Job::new(12, 10, 0);
    let job_queue = JobQueue::init_from(vec![job1, job2]);
    let simulator = Simulation::new(manager, processor, job_queue);
    let final_frame = simulator.play_to_end();

    // test that the final frame has the jobs in the right places
    assert_eq!(job1, final_frame.jobs[0]);
    assert_eq!(job2, final_frame.jobs[1]);
    assert_eq!(None, final_frame.remaining_jobs());
}
