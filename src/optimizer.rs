use crate::objects::{Schedule, Scene};
use std::collections::HashMap;
use std::collections::VecDeque;
use rand::Rng;

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

    pub fn local_optimization(&mut self, breakpoint: u32, verbose: bool) -> u32 {
        let len = self.schedule.scenes.len();
        let mut times_without_improvement = 0;

        let mut previous_best_cost = u32::MAX;
        let mut local_best_cost = u32::MAX;
        let mut local_best_order: Vec<u32> = vec![];

        
        loop {
            self.schedule.calculate_cost();
            if verbose{ 
                println!("SOLIS");
                self.schedule.print_short();
            };

            if self.schedule.cost < local_best_cost {
                local_best_cost = self.schedule.cost;
                local_best_order = self.scenes_to_ids();
            }
            for i in 1..len {
                //Samainam vietām blakus esošās ainas
                self.schedule.scenes.swap(i-1, i);
                self.schedule.calculate_cost();
                
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
        self.schedule.cost
    }

    pub fn late_acceptance_hillclimbing(&mut self, memory_length: usize, findable_optimum_count: u32, max_cycles: u32, verbose: bool) -> u32 {
        let mut rng = rand::thread_rng();
        let len = self.schedule.scenes.len();

        //Turam labākās vērtības
        let mut best_cost = self.schedule.calculate_cost();
        let mut best_order = self.scenes_to_ids();
        let mut times_best_found = 0;
        let mut times_ran = 0;


        //Saglabājam vēsturi
        let mut previous_que = VecDeque::new();
        previous_que.push_back(best_cost);

        //Iterējam variantus
        loop {
            times_ran += 1;
            let last_cost = *previous_que.back().unwrap();
            let oldest_cost = *previous_que.front().unwrap();

            //Samainam 2 random elementus
            let pos1 = rng.gen_range(0..len);
            let mut pos2 = rng.gen_range(0..len);
            while pos1 == pos2 {
                pos2 = rng.gen_range(0..len);
            }
            self.swap_items(pos1, pos2);

            //Aprēķinam izmaksu variantam
            let current_cost = self.schedule.calculate_cost();
            
            //Ja ir dārgāks par vecāko un iepriekšējo, atgriežam iepriekšējā vērtībā
            if current_cost > oldest_cost && current_cost > last_cost {
                self.swap_items(pos1,pos2);
                self.schedule.calculate_cost();
            }

            //Saglabājam labākos variantus, lai ja variants ar tādu pašu vērtību atrasts X reizes, varam atgriezties
            if current_cost == best_cost {
                times_best_found += 1;
                best_order = self.scenes_to_ids();
                if verbose {print!(" + {}", times_best_found)};
            } else if current_cost < best_cost {
                if verbose {print!("\nNEWBESTFOUND {}", current_cost)};
                times_best_found = 0;
                best_cost = current_cost;
                best_order = self.scenes_to_ids();
            }

            //Atjaunojam atmiņu
            if previous_que.len() >= memory_length {
                previous_que.pop_front();
            }
            previous_que.push_back(current_cost);

            // Atgriežamies
            if times_best_found >= findable_optimum_count || times_ran > max_cycles{
                break;
            }
        }

        //Atjaunojam labāko, printējam

        if times_ran > max_cycles{
            if verbose{ println!("\nPĀRTRAUKTS TIMEOUT DĒĻ")};
        }
        self.ids_to_scenes(&best_order);
        self.schedule.calculate_cost();
        self.print_best();
        self.schedule.cost
    }


    pub fn print_best(&self) {
        println!("\nLABĀKAIS\n---------------------");
        self.schedule.print_short()
    }

    pub fn print_best_full(&self) {
        println!("\nLABĀKAIS\n---------------------");
        self.schedule.print()
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

    fn swap_items(&mut self, pos1: usize, pos2: usize){
        self.schedule.scenes.swap(pos1, pos2);
        self.scene_positions.insert(self.schedule.scenes[pos1].id, pos1);
        self.scene_positions.insert(self.schedule.scenes[pos2].id, pos2);
    }

}
