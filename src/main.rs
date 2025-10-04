mod objects;
mod optimizer;
mod scenarios;
use optimizer::Optimizer;
use scenarios::{hardcoded1_schedule, hardcoded2_schedule, hardcoded_simple_schedule, randomized_schedule};
use objects::Schedule;
use std::time::Instant;

fn main() {
    let mut schedule = hardcoded_simple_schedule();
    test_optimization(schedule, false);

    let mut schedule2 = hardcoded1_schedule();
    test_optimization(schedule2, false);

    let mut schedule3 = hardcoded2_schedule();
    test_optimization(schedule3, false);

    let mut schedule4 = randomized_schedule(100,200);
    test_optimization(schedule4, true);
}

fn test_optimization(schedule: Schedule, verbose: bool){
    let mut optimizer = Optimizer::new(schedule.clone());
    let mut optimizer2 = Optimizer::new(schedule.clone());
   
   
    let timer_local = Instant::now();
    let local_best = optimizer.local_optimization(100, verbose);
    let local_time = timer_local.elapsed();
    
    let timer_hillclimbing = Instant::now();
    let hillclimbing_best = optimizer2.late_acceptance_hillclimbing(10, 5, 800000, verbose);
    let hillclimbing_time = timer_hillclimbing.elapsed();

    println!("LOKĀLAIS: {} cost, {:?} time; HILLCLIMBGING: {} cost, {:?} time;", local_best, local_time, hillclimbing_best, hillclimbing_time);
}