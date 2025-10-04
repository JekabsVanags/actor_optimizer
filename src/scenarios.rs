use crate::objects::{Actor, Scene, Schedule};
use rand::Rng;

pub fn hardcoded_simple_schedule() -> Schedule{
  let mut actors = vec![];
  actors.push(Actor::new(1, "Alice".to_string(), 10));
  actors.push(Actor::new(2, "Bob".to_string(), 20));
  actors.push(Actor::new(3, "Jack".to_string(), 100));
  actors.push(Actor::new(4, "Ann".to_string(), 20));
  actors.push(Actor::new(5, "Josh".to_string(), 10));
  

  // Create scenes
  let scene1 = Scene::new(1, "Opening Scene".to_string(), vec![1,2,3]);
  let scene2 = Scene::new(2, "Scene 2".to_string(), vec![2, 5]);
  let scene3 = Scene::new(3, "Scene 3".to_string(), vec![1, 5]);
  let scene4 = Scene::new(4, "Closing Scene".to_string(), vec![3,4]);

  Schedule::new(vec![scene1, scene2, scene3, scene4], actors)
  //LABĀKĀS IZMAKSAS 100 * 2 + 20 * 1 + 20 * 2 + 10 * 3 + 10 * 2 = 310
  //SECIBA 4, 1, 2, 3  (viena no)
}

//Viens dārgs aktieris "jāoptimizē" kopā
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
  
  //LABĀKĀS IZMAKSAS 10 * 4 + 20 * 4 + 20 * 1 + 10 * 3 + 100 * 4 + 10 * 2 = 590
  //SECIBA 1, 5, 3, 6, 4, 2 (viena no)
}

//Vairāki vienlīdz dārgi aktieri pārklājas
pub fn hardcoded2_schedule() -> Schedule {
    let mut actors = vec![];
    actors.push(Actor::new(1, "Alice".to_string(), 20));
    actors.push(Actor::new(2, "Bob".to_string(), 80));
    actors.push(Actor::new(3, "Jack".to_string(), 80));
    actors.push(Actor::new(4, "Ann".to_string(), 25));
    actors.push(Actor::new(5, "Josh".to_string(), 25));

    // Scenes designed to create optimization conflict
    let scene1 = Scene::new(1, "Scene 1".to_string(), vec![1,3]);      
    let scene2 = Scene::new(2, "Scene 2".to_string(), vec![2,3]);
    let scene3 = Scene::new(3, "Scene 3".to_string(), vec![4,5]); 
    let scene4 = Scene::new(4, "Scene 4".to_string(), vec![1,2]);
    let scene5 = Scene::new(5, "Scene 5".to_string(), vec![3,4]);
    let scene6 = Scene::new(6, "Scene 6".to_string(), vec![5,1]);

    Schedule::new(vec![scene1, scene2, scene3, scene4, scene5, scene6], actors)
    
    //LABĀKĀS IZMAKSAS 20 * 6 + 80 * 2 + 80 * 3 + 25 * 2 + 25 * 2 = 620
    //SECIBA 4, 2, 1, 5, 3, 6 (viena no)
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
          let actor_id = rng.gen_range(1..=actor_size);
          if !scene_actor_ids.contains(&actor_id) {
              scene_actor_ids.push(actor_id);
          }
      }
      scenes.push(Scene::new(id, name, scene_actor_ids));
  }

  Schedule::new(scenes, actors)
}