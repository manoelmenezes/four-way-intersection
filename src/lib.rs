use std::{thread, time};

pub struct Controller {
    vertical_car_traffic_lights: Vec<TrafficLight>,
    horizontal_car_traffic_lights: Vec<TrafficLight>,
    vertical_pedestrian_traffic_lights: Vec<TrafficLight>,
    horizontal_pedestrian_traffic_lights: Vec<TrafficLight>,
    car_green_time: u64,
    car_yellow_time: u64,
    car_red_to_green_transition_time: u64,
    pedestrian_green_time: u64,
    pedestrian_yellow_time: u64,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            vertical_car_traffic_lights: vec![TrafficLight::new(), TrafficLight::new()],
            horizontal_car_traffic_lights: vec![TrafficLight::new(), TrafficLight::new()],
            vertical_pedestrian_traffic_lights: vec![
                                                    TrafficLight::new(), TrafficLight::new(),
                                                    TrafficLight::new(), TrafficLight::new()
                                                ], 
            horizontal_pedestrian_traffic_lights: vec![
                                                    TrafficLight::new(), TrafficLight::new(),
                                                    TrafficLight::new(), TrafficLight::new()
                                                  ],
            car_green_time: 30,
            car_yellow_time: 5,
            car_red_to_green_transition_time: 5,
            pedestrian_green_time: 10,
            pedestrian_yellow_time: 5,
        }
    }

    pub fn start(&mut self) {
        loop {
            for tl in &mut self.horizontal_pedestrian_traffic_lights {
                tl.green();
            }
            println!("Horizontal pedestrian lights transitioned to green state.");
            thread::sleep(time::Duration::from_secs(self.car_red_to_green_transition_time));

            for tl in &mut self.horizontal_car_traffic_lights {
                tl.green();
            }
            println!("Horizontal car traffic lights transitioned to green state.");

            let pgt = self.pedestrian_green_time - self.car_red_to_green_transition_time;
            thread::sleep(time::Duration::from_secs(pgt));

            for tl in &mut self.horizontal_pedestrian_traffic_lights {
                tl.yellow();
            }
            println!("Horizontal pedestrian lights transitioned to yellow state.");
            thread::sleep(time::Duration::from_secs(self.pedestrian_yellow_time));

            for tl in &mut self.horizontal_pedestrian_traffic_lights {
                tl.red();
            }
            println!("Horizontal pedestrian lights transitioned to red state.");
            thread::sleep(time::Duration::from_secs(self.car_green_time - self.pedestrian_yellow_time - pgt));

            for tl in &mut self.horizontal_car_traffic_lights {
                tl.yellow();
            }
            println!("Horizontal car traffic lights transitioned to yellow state.");
            thread::sleep(time::Duration::from_secs(self.car_yellow_time));

            for tl in &mut self.horizontal_car_traffic_lights {
                tl.red();
            }
            println!("Horizontal car traffic lights transitioned to red state.");
            
            for tl in &mut self.vertical_pedestrian_traffic_lights {
                tl.green();
            }
            println!("Vertical pedestrian lights transitioned to green state.");
            thread::sleep(time::Duration::from_secs(self.car_red_to_green_transition_time));

            for tl in &mut self.vertical_car_traffic_lights {
                tl.green();
            }
            println!("Vertical car traffic lights transitioned to green state.");

            let pgt = self.pedestrian_green_time - self.car_red_to_green_transition_time;
            thread::sleep(time::Duration::from_secs(pgt));

            for tl in &mut self.vertical_pedestrian_traffic_lights {
                tl.yellow();
            }
            println!("Vertical pedestrian lights transitioned to yellow state.");
            thread::sleep(time::Duration::from_secs(self.pedestrian_yellow_time));

            for tl in &mut self.vertical_pedestrian_traffic_lights {
                tl.red();
            }
            println!("Vertical pedestrian lights transitioned to red state.");
            thread::sleep(time::Duration::from_secs(self.car_green_time - self.pedestrian_yellow_time - pgt));

            for tl in &mut self.vertical_car_traffic_lights {
                tl.yellow();
            }
            println!("Vertical car traffic lights transitioned to yellow state.");
            thread::sleep(time::Duration::from_secs(self.car_yellow_time));

            for tl in &mut self.vertical_car_traffic_lights {
                tl.red();
            }
            println!("Vertical car traffic lights transitioned to red state.");
        }
    }
}

enum State {
    Green,
    Yellow,
    Red,
}

struct TrafficLight {
    state: State, 
}

impl TrafficLight {
    fn new() -> Self {
        Self {
            state: State::Red,
        }
    }

    fn green(&mut self) {
        self.state = State::Green;
    }

    fn yellow(&mut self) {
        self.state = State::Yellow;
    }

    fn red(&mut self) {
        self.state = State::Red;
    }
}

