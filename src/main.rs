mod objects;
use objects::Actor;
use objects::Scene;
use objects::Schedule;

fn main() {
    // Create some actors
    let mut actor1 = Actor::new(1, "Alice".to_string(), 25);
    let mut actor2 = Actor::new(2, "Bob".to_string(), 30);
    let mut actor3 = Actor::new(3, "Jack".to_string(), 40);

    // Add some days on set
    actor1.add_days_on_set(2);
    actor2.add_days_on_set(5);

    // Create scenes
    let scene1 = Scene::new(1, "Opening Scene".to_string(), vec![actor1, actor2]);
    let scene2 = Scene::new(2, "Closing Scene".to_string(), vec![actor3]);

    // Create schedule
    let schedule = Schedule::new(vec![scene1, scene2]);

    // Print schedule
    schedule.print();
}
