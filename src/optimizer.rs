use crate::objects::{Schedule, Scene};
use std::collections::HashMap;

pub struct Optimizer {
    schedule: Schedule,
    scene_positions: HashMap<u32, usize>
}

impl Optimizer {
    pub fn new(schedule: Schedule) -> Self {
        //Sagatavojam sarakstu kura id ainas ir kurā vietā
        let mut scene_positions = HashMap::new();
        for (i, scene) in schedule.scenes.iter().enumerate() {
            scene_positions.insert(scene.id, i);
        }

        Self {schedule, scene_positions}
    }

    pub fn generate(&mut self, breakpoint: u32) {
        let len = self.schedule.scenes.len();
        let mut times_without_improvement = 0;

        let mut previous_best_cost = u32::MAX;
        let mut local_best_cost = u32::MAX;
        let mut local_best_order: Vec<u32> = vec![];

        
        loop {
            println!("SOLIS");
            self.schedule.calculate_cost();
            self.schedule.print_short();

            if self.schedule.cost < local_best_cost {
                local_best_cost = self.schedule.cost;
                local_best_order = self.scenes_to_ids();
            }
            for i in 1..len {
                //Samainam vietām blakus esošās ainas
                self.schedule.scenes.swap(i-1, i);
                self.schedule.calculate_cost();
                //self.schedule.print_short();

                if self.schedule.cost < local_best_cost {
                    local_best_cost = self.schedule.cost;
                    local_best_order = self.scenes_to_ids();   
                }

                self.schedule.scenes.swap(i-1, i);
            }
            
            self.ids_to_scenes(&local_best_order);
            self.schedule.calculate_cost();
            //Pārbaudam beigu notiekumu
            if local_best_cost >= previous_best_cost {
                times_without_improvement += 1;
                if times_without_improvement >= breakpoint{
                    break;
                }
            } else {
                times_without_improvement = 0;
            }
            previous_best_cost = local_best_cost;
        }
        self.schedule.calculate_cost();
        self.print_best();
    }

    pub fn print_best(&self) {
        println!("\nLABĀKAIS\n---------------------");
        self.schedule.print_short()
    }

    fn scenes_to_ids(&self) -> Vec<u32> {
        self.schedule.scenes.iter().map(|s| s.id).collect()
    }

    //Sakārtojam ainas pēc id saraksta
    fn ids_to_scenes(&mut self, target_order: &[u32]) {
        for (new_index, &scene_id) in target_order.iter().enumerate() {
        let current_index = self.scene_positions[&scene_id]; 
        self.schedule.scenes.swap(new_index, current_index);
        // Atjaunojam karšu informāciju par jaunajām pozīcijām
        self.scene_positions.insert(self.schedule.scenes[current_index].id, current_index);
        self.scene_positions.insert(self.schedule.scenes[new_index].id, new_index);
        }
    }

}
