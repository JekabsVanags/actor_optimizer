mod objects;
mod optimizer;
use objects::{Actor, Scene, Schedule};
use optimizer::Optimizer;

fn main() {
    // Create some actors
    let mut actors = vec![];
    actors.push(Actor::new(1, "Alice".to_string(), 10));
    actors.push(Actor::new(2, "Bob".to_string(), 10));
    actors.push(Actor::new(3, "Jack".to_string(), 10));
    actors.push(Actor::new(4, "Ann".to_string(), 10));

    // Create scenes
    let scene1 = Scene::new(1, "Opening Scene".to_string(), vec![1,2]);
    let scene2 = Scene::new(2, "Scene 2".to_string(), vec![3]);
    let scene3 = Scene::new(3, "Scene 3".to_string(), vec![3,2]);
    let scene4 = Scene::new(4, "Closing Scene".to_string(), vec![3,4]);

    // Create schedule
    let mut schedule = Schedule::new(vec![scene1, scene2, scene3, scene4], actors);
    // Print schedule
    schedule.calculate_cost();
    schedule.print();

    let mut optimizer = Optimizer::new(schedule);
    optimizer.generate(10);
}
