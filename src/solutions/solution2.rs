use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum State {
    Green,
    Yellow,
    Red,
}

struct Controller {}

impl Controller {

    pub fn start(&self) {
    }


    fn start_traffic_light(mut tl:TrafficLight, rx: Receiver<State>) {
        thread::spawn(move || {
            for state in rx {
                tl.set_state(state);
                println!("{} transitioned to {:#?}.", tl.id, tl.state);
            }
        });
    }

}

struct TrafficLight {
    id: String,
    state: State,
}

impl TrafficLight {

    fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

