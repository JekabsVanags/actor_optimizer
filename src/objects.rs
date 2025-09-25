pub struct Actor {
    pub id: u32,
    pub name: String,
    pub vage: u32,
    pub days_on_set: u32
}

impl Actor {
    pub fn new(id: u32, name: String, vage: u32) -> Self {
        Self { id, name, vage, days_on_set: 0 }
    }

    pub fn add_days_on_set(&mut self, days_added: u32) {
        self.days_on_set += days_added;
    }
}

pub struct Scene {
    pub id: u32,
    pub title: String,
    pub actors: Vec<Actor>,
}

impl Scene {
    pub fn new(id: u32, title: String, actors: Vec<Actor>) -> Self {
        Self { id, title, actors }
    }
}

pub struct Schedule {
    pub scenes: Vec<Scene>,
    pub cost: u32
}

impl Schedule {
    pub fn new(scenes: Vec<Scene>)-> Self {
        let mut schedule = Self {scenes, cost: 0};
        schedule.calculate_cost();
        schedule
    }

    pub fn print(&self){
        println!("IZMAKSAS: {}", self.cost);
        println!("GRAFIKS:");
        for scene in &self.scenes{
            print!("{}, actors: ", scene.title);
            for actor in &scene.actors{
                print!("{}, ", actor.name)
            }
            println!("");
        }
    }

    fn calculate_cost(&mut self){
        let mut calculated_cost = 0;
        for scene in &self.scenes {
            for actor in &scene.actors {
                calculated_cost += actor.vage;
            }
        }
        self.cost = calculated_cost;
    }
}