use std::collections::HashMap;


#[derive(Clone)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub wage: u32,
    //Palīgvērtības ātrākām kalkulācijām
    pub first_day: u32,
    pub last_day: u32
}

impl Actor {
    pub fn new(id: u32, name: String, wage: u32) -> Self {
        Self { id, name, wage, first_day: 0, last_day: 0 }
    }
}


#[derive(Clone)]
pub struct Scene {
    pub id: u32,
    pub title: String,
    pub actor_ids: Vec<u32>,
}

impl Scene {
    pub fn new(id: u32, title: String, actor_ids: Vec<u32>) -> Self {
        Self { id, title, actor_ids }
    }
}


#[derive(Clone)]
pub struct Schedule {
    pub scenes: Vec<Scene>,
    pub actors: HashMap<u32, Actor>,
    pub cost: u32
}

impl Schedule {
    pub fn new(scenes: Vec<Scene>, actors: Vec<Actor>) -> Self {
        let actors_map: HashMap<u32, Actor> = actors.into_iter().map(|a| (a.id, a)).collect();
        Self { scenes, actors: actors_map, cost: 0 }
    }

    pub fn reset_actors(&mut self) {
        for actor in self.actors.values_mut() {
            actor.first_day = 0;
            actor.last_day = 0;
        }
    }

    pub fn calculate_cost(&mut self) -> u32 {
        let mut calculated_cost = 0;

        self.reset_actors();

        for (scene_index, scene) in self.scenes.iter().enumerate() {
            let day_number = (scene_index + 1) as u32;

            for actor_id in &scene.actor_ids {
                if let Some(actor) = self.actors.get_mut(actor_id) {
                    if actor.first_day == 0 {
                        actor.first_day = day_number;
                        actor.last_day = day_number;
                        calculated_cost += actor.wage;
                    } else if actor.last_day < day_number {
                        let days_on_set = day_number - actor.last_day;
                        calculated_cost += actor.wage * days_on_set;
                        actor.last_day = day_number;
                    }
                }
            }
        }

        self.cost = calculated_cost;
        calculated_cost
    }

    pub fn print(&self) {
        println!("IZMAKSAS: {}", self.cost);
        println!("GRAFIKS:");

        for scene in &self.scenes {
            print!("{}, actors: ", scene.title);
            for actor_id in &scene.actor_ids {
                if let Some(actor) = self.actors.get(actor_id) {
                    print!("{}, ", actor.name);
                } else {
                    print!("Unknown actor (ID {}), ", actor_id);
                }
            }
            println!();
        }
        println!("--------------------")
    }

    pub fn print_short(&self){
        print!("IZMAKSAS: {}, SECĪBA: [", self.cost);
        for scene in &self.scenes {
            print!("{}, ", scene.id);
        }
        println!("]")
    }
}
