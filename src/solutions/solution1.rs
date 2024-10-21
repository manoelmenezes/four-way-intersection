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

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
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

    fn control(
        &mut self,
        direction: Direction 
    ) {
        let (pedestrian_traffic_lights, car_traffic_lights) = match direction {
            Direction::Horizontal => (
                                         &mut self.horizontal_pedestrian_traffic_lights,
                                         &mut self.horizontal_car_traffic_lights
                                     ),
            Direction::Vertical => (
                                       &mut self.vertical_pedestrian_traffic_lights,
                                       &mut self.vertical_car_traffic_lights
                                   ),
        };

        Controller::change_state(pedestrian_traffic_lights, State::Green);
        println!("{:#?} pedestrian lights transitioned to green state.", direction);
        thread::sleep(time::Duration::from_secs(self.car_red_to_green_transition_time));

        Controller::change_state(car_traffic_lights, State::Green);
        println!("{:#?} car traffic lights transitioned to green state.", direction);

        let pgt = self.pedestrian_green_time - self.car_red_to_green_transition_time;
        thread::sleep(time::Duration::from_secs(pgt));

        Controller::change_state(pedestrian_traffic_lights, State::Yellow);
        println!("{:#?} pedestrian lights transitioned to yellow state.", direction);
        thread::sleep(time::Duration::from_secs(self.pedestrian_yellow_time));

        Controller::change_state(pedestrian_traffic_lights, State::Red);
        println!("{:#?} pedestrian lights transitioned to red state.", direction);
        thread::sleep(time::Duration::from_secs(self.car_green_time - self.pedestrian_yellow_time - pgt));

        Controller::change_state(car_traffic_lights, State::Yellow);
        println!("{:#?} car traffic lights transitioned to yellow state.", direction);
        thread::sleep(time::Duration::from_secs(self.car_yellow_time));

        Controller::change_state(car_traffic_lights, State::Red);
        println!("{:#?} car traffic lights transitioned to red state.", direction);
    }

    fn change_state(traffic_lights: &mut Vec<TrafficLight>, state: State) {
        match state {
            State::Green => for tl in traffic_lights { tl.green() },
            State::Yellow => for tl in traffic_lights { tl.yellow() },
            State::Red => for tl in traffic_lights { tl.red() }, 
        }
    }

    pub fn start(&mut self) {
        loop {
            self.control(Direction::Horizontal);
            self.control(Direction::Vertical);
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
