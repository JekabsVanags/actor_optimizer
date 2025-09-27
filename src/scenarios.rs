use crate::objects::{Actor, Scene, Schedule};
use rand::Rng;

pub fn hardcoded1_schedule() -> Schedule{
  // Create some actors
  let mut actors = vec![];
  actors.push(Actor::new(1, "Alice".to_string(), 10));
  actors.push(Actor::new(2, "Bob".to_string(), 20));
  actors.push(Actor::new(3, "Jack".to_string(), 100));
  actors.push(Actor::new(4, "Ann".to_string(), 30));
  actors.push(Actor::new(5, "Josh".to_string(), 20));
  actors.push(Actor::new(6, "Emanuel".to_string(), 10));
  

  // Create scenes
  let scene1 = Scene::new(1, "Opening Scene".to_string(), vec![1,2,5]);
  let scene2 = Scene::new(2, "Scene 2".to_string(), vec![3]);
  let scene3 = Scene::new(3, "Scene 3".to_string(), vec![3,2,6]);
  let scene4 = Scene::new(4, "Closing Scene".to_string(), vec![3,4]);
  let scene5 = Scene::new(5, "Afterwords".to_string(), vec![2,6,1]);
  let scene6 = Scene::new(6, "Closing Credits".to_string(), vec![3,1,2]);

  Schedule::new(vec![scene1, scene2, scene3, scene4, scene5, scene6], actors)
}

pub fn randomized_schedule(actor_size: u32, scene_size: u32) -> Schedule{
  let mut rng = rand::thread_rng();

  // Generējam akrierus
  let mut actors = vec![];
  for id in 1..=actor_size {
      let name = format!("Actor{}", id);
      let cost = rng.gen_range(5..=100);
      actors.push(Actor::new(id, name, cost));
  }

  // Ģenerējam ainas
  let mut scenes = vec![];
  for id in 1..=scene_size {
      let name = format!("Scene {}", id);
      // Randomly choose 2 to 5 actors for this scene
      let num_actors = rng.gen_range(2..=10);
      let mut scene_actor_ids = vec![];
      while scene_actor_ids.len() < num_actors {
          let actor_id = rng.gen_range(1..=50);
          if !scene_actor_ids.contains(&actor_id) {
              scene_actor_ids.push(actor_id);
          }
      }
      scenes.push(Scene::new(id, name, scene_actor_ids));
  }

  Schedule::new(scenes, actors)
}