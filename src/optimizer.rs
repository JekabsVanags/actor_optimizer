use crate::objects::{Schedule};
use rand::Rng;


pub struct Optimizer{
    schedule: Schedule,
    best_scene_order: Vec<u32>,
    best_scene_cost: u32
}

impl Optimizer{
    pub fn new(schedule: Schedule) -> Self{
        Self{schedule, best_scene_order: vec![], best_scene_cost: u32::MAX}
    }

    pub fn generate(&mut self, times: u32){
        for _ in 0..times{
            self.schedule.reset_actors();

            let len = self.schedule.scenes.len();
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..len);
            let j = rng.gen_range(0..len);

            self.schedule.scenes.swap(i, j);
            self.schedule.calculate_cost();
            self.schedule.print_short();


            if self.best_scene_cost > self.schedule.cost {
                self.best_scene_cost = self.schedule.cost;
                let mut best_order = vec![];
                for scene in &self.schedule.scenes{
                    best_order.push(scene.id);
                }
                self.best_scene_order = best_order;
            }
        }

        self.print_best();
    }

    pub fn print_best(&self){
        println!("\nLABĀKAIS\n---------------------");
        print!("CENA: {}, SECĪBA: [", self.best_scene_cost);
        for scene in &self.best_scene_order{
            print!("{}, ", scene);
        }
        println!("]")
    }
}