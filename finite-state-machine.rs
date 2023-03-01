//! A basic traffic light state machine.
//!
//! Wikipedia definition of a FSM:
//! A finite state machine can be in exactly one of a finite number of states at any given time.
//! The FSM can change from one state to another in response to some inputs; the change from one state to another is called a transition.
//! An FSM is defined by a list of its states, its initial state, and the inputs that trigger each transition.
//!
//! Tested with rust-1.6.4
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2023-01-05

#[derive(Debug)]
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self::Green
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Green => Self::Yellow,
            Self::Yellow => Self::Red,
            Self::Red => Self::Green,
        }
    }
}

#[derive(Debug)]
struct Crosswalk {
    light: TrafficLight,
}

impl Crosswalk {
    pub fn new() -> Self {
        Crosswalk {
            light: TrafficLight::new(),
        }
    }
    pub fn next(&mut self) {
        self.light = self.light.next();
        self.print();
    }
    pub fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut crosswalk = Crosswalk::new();
    crosswalk.print();
    crosswalk.next();
    crosswalk.next();
    crosswalk.next();
}
