mod objects;
mod optimizer;
mod scenarios;
use optimizer::Optimizer;
use scenarios::{hardcoded1_schedule, randomized_schedule};

fn main() {
    // let mut schedule = hardcoded1_schedule();
    // let mut optimizer = Optimizer::new(schedule);
    // optimizer.generate(10);

    let mut schedule2 = randomized_schedule(300,200);
    let mut optimizer = Optimizer::new(schedule2);
    optimizer.generate(100);
}