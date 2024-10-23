use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
enum State {
    Green,
    Yellow,
    Red,
}

enum Direction {
    Horizontal,
    Vertical,
}

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
            vertical_car_traffic_lights: vec![
                TrafficLight::new(String::from("Vertical-Car-Up")),
                TrafficLight::new(String::from("Vertical-Car-Down"))
            ],
            horizontal_car_traffic_lights: vec![
                TrafficLight::new(String::from("Horizontal-Car-Left")),
                TrafficLight::new(String::from("Horizontal-Car-Right"))
            ],
            vertical_pedestrian_traffic_lights: vec![
                TrafficLight::new(String::from("Vertical-Pedestrian-Up-Left")),
                TrafficLight::new(String::from("Vertical-Pedestrian-Up-Right")),
                TrafficLight::new(String::from("Vertical-Pedestrian-Down-Left")),
                TrafficLight::new(String::from("Vertical-Pedestrian-Down-Right"))
            ],
            horizontal_pedestrian_traffic_lights: vec![
                TrafficLight::new(String::from("Horizontal-Pedestrian-Up-Left")),
                TrafficLight::new(String::from("Horizontal-Pedestrian-Up-Right")),
                TrafficLight::new(String::from("Horizontal-Pedestrian-Down-Left")),
                TrafficLight::new(String::from("Horizontal-Pedestrian-Down-Right"))
            ],
            car_green_time: 30,
            car_yellow_time: 5,
            car_red_to_green_transition_time: 5,
            pedestrian_green_time: 10,
            pedestrian_yellow_time: 5,
        }
    }

    pub fn start(&self) {
        let start_result = self.start_traffic_lights();        
        loop {        
            self.control(&start_result, Direction::Horizontal);
            self.control(&start_result, Direction::Vertical);
        }
    }

    fn control(&self, start_result: &StartTrafficLightsResult, direction: Direction) {
        let (pedestrian_senders, car_senders) = match direction {
            Direction::Horizontal => (
                                         &start_result.horizontal_pedestrian_senders,
                                         &start_result.horizontal_car_senders
                                     ),
            Direction::Vertical => (
                                       &start_result.vertical_pedestrian_senders,
                                       &start_result.vertical_car_senders
                                   ),
        };
        Controller::change_state(pedestrian_senders, State::Green);
        //println!("{:#?} pedestrian lights transitioned to green state.", direction);
        thread::sleep(Duration::from_secs(self.car_red_to_green_transition_time));

        Controller::change_state(car_senders, State::Green);
        //println!("{:#?} car traffic lights transitioned to green state.", direction);

        let pgt = self.pedestrian_green_time - self.car_red_to_green_transition_time;
        thread::sleep(Duration::from_secs(pgt));

        Controller::change_state(pedestrian_senders, State::Yellow);
        //println!("{:#?} pedestrian lights transitioned to yellow state.", direction);
        thread::sleep(Duration::from_secs(self.pedestrian_yellow_time));

        Controller::change_state(pedestrian_senders, State::Red);
        //println!("{:#?} pedestrian lights transitioned to red state.", direction);
        thread::sleep(Duration::from_secs(self.car_green_time - self.pedestrian_yellow_time - pgt));

        Controller::change_state(car_senders, State::Yellow);
        //println!("{:#?} car traffic lights transitioned to yellow state.", direction);
        thread::sleep(Duration::from_secs(self.car_yellow_time));

        Controller::change_state(car_senders, State::Red);
        //println!("{:#?} car traffic lights transitioned to red state.", direction);
    }

    fn change_state(senders: &Vec<Sender<State>>, state: State) {
        for tx in senders {
            tx.send(state.clone()).unwrap();
        }
    }

    fn start_traffic_lights(&self) -> StartTrafficLightsResult {
        
        let horizontal_pedestrian_senders = self.do_start_traffic_lights(&self.horizontal_pedestrian_traffic_lights);
        let horizontal_car_senders = self.do_start_traffic_lights(&self.horizontal_car_traffic_lights);
        let vertical_pedestrian_senders = self.do_start_traffic_lights(&self.vertical_pedestrian_traffic_lights);
        let vertical_car_senders = self.do_start_traffic_lights(&self.vertical_car_traffic_lights);

        StartTrafficLightsResult {
            horizontal_pedestrian_senders,
            horizontal_car_senders,
            vertical_pedestrian_senders,
            vertical_car_senders,
        }
    }

    fn do_start_traffic_lights(&self, traffic_lights: &Vec<TrafficLight>) -> Vec<Sender<State>> {
        let mut senders = vec![];
        for tl in traffic_lights {
            let (tx, rx) = mpsc::channel();
            Controller::start_traffic_light(tl.clone(), rx);
            senders.push(tx);
        }
        senders
    }


    fn start_traffic_light(mut tl: TrafficLight, rx: Receiver<State>) {
        thread::spawn(move || {
            for state in rx {
                tl.set_state(state);
                println!("{} transitioned to {:#?}.", tl.id, tl.state);
            }
        });
    }

}

struct StartTrafficLightsResult {
    horizontal_pedestrian_senders: Vec<Sender<State>>,
    horizontal_car_senders: Vec<Sender<State>>,
    vertical_pedestrian_senders: Vec<Sender<State>>,
    vertical_car_senders: Vec<Sender<State>>,
}

#[derive(Clone)]
struct TrafficLight {
    id: String,
    state: State,
}

impl TrafficLight {

    fn new(id: String) -> Self {
        Self {
            id,
            state: State::Red,
        }
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

