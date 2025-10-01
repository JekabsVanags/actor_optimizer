mod objects;
mod optimizer;
mod scenarios;
use optimizer::Optimizer;
use scenarios::{hardcoded1_schedule, hardcoded_simple_schedule, randomized_schedule};

fn main() {
    // let mut schedule = hardcoded_simple_schedule();
    // let mut optimizer = Optimizer::new(schedule);
    // let local_best = optimizer.local_optimization(100);
    // let hillclimbing_best = optimizer.late_acceptance_hillclimbing(5, 10, 20000);

    // println!("LOKĀLAIS: {}, HILLCLIMBGING: {}", local_best, hillclimbing_best);
    // optimizer.print_best_full();

    let mut schedule2 = randomized_schedule(100,50);
    let mut optimizer = Optimizer::new(schedule2.clone());
    let local_best = optimizer.local_optimization(100);
    let mut optimizer2 = Optimizer::new(schedule2.clone());
    let hillclimbing_best = optimizer2.late_acceptance_hillclimbing(5, 3, 800000);

    println!("LOKĀLAIS: {}, HILLCLIMBGING: {}", local_best, hillclimbing_best);

    optimizer2.print_best_full();
}