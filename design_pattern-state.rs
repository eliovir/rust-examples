//! State pattern.
//!
//! Design pattern where the objects control their behavior by changing their internal state.
//! The objects delegate the work to the State.
//! Localize the behavior in structs and making things a lot easier to change and understand.
//!
//! Tested with rust-1.6.4
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2023-02-28
extern crate rand;

use std::fmt;
use rand::Rng;

/// State trait that contains a method for every action in the Gumball Machine.
trait GumballMachineAction {
    fn insert_quarter(&self, m: &mut GumballMachine);
    fn eject_quarter(&self, m: &mut GumballMachine);
    fn turn_crank(&self, m: &mut GumballMachine);
    // Internal action the machine invokes on itself.
    // Test for zero or more gumballs in the “Gumball Sold” state,
    // and then either go to the “Out of Gumballs” state or the “No Quarter” state.
    fn dispense(&self, m: &mut GumballMachine);
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized;
}

/// State structs for every state of the machine.
/// These classes will be responsible for the behavior of the machine when it is in the corresponding state.

#[derive(Clone, Copy, Debug)]
struct NoQuarterState;

/// The starting state for the gumball machine.
#[derive(Clone, Copy, Debug)]
struct HasQuarterState;
/// 10% of the time when the crank is turned, the customer get two gumballs instead of one
#[derive(Clone, Copy, Debug)]
struct WinnerState;
#[derive(Clone, Copy, Debug)]
struct GumballSoldState;
#[derive(Clone, Copy, Debug)]
struct OutOfGumballsState;

#[derive(Clone, Copy)]
enum GumballMachineState {
    NoQuarter,
    HasQuarter,
    GumballSold,
    OutOfGumballs,
    Winner
}
impl fmt::Display for GumballMachineState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            GumballMachineState::NoQuarter => "NoQuarter",
            GumballMachineState::HasQuarter => "HasQuarter",
            GumballMachineState::GumballSold => "GumballSold",
            GumballMachineState::OutOfGumballs => "OutOfGumballs",
            GumballMachineState::Winner => "Winner"
        };
		write!(f, "{}", repr)
	}
}

fn from(state: GumballMachineState) -> Box<dyn GumballMachineAction> {
    match state {
        GumballMachineState::NoQuarter => NoQuarterState::new(),
        GumballMachineState::HasQuarter => HasQuarterState::new(),
        GumballMachineState::GumballSold => GumballSoldState::new(),
        GumballMachineState::OutOfGumballs => OutOfGumballsState::new(),
        GumballMachineState::Winner => WinnerState::new(),
    }
}

impl GumballMachineAction for NoQuarterState {
    fn insert_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you inserted a quarter");
        m.set_state(GumballMachineState::HasQuarter);
    }
    fn eject_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot eject quarter as there is no quarter");
    }
    fn turn_crank(&self, m: &mut GumballMachine) {
        m.display_message("you turned crank, but there is no quarter");
    }
    fn dispense(&self, m: &mut GumballMachine) {
        m.display_message("you need to insert a quarter to dispense a gumball");
    }
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized {
        Box::new(NoQuarterState {})
    }
}
impl GumballMachineAction for HasQuarterState {
    fn insert_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot insert quarter as there is already a quarter");
    }
    fn eject_quarter(&self, m: &mut GumballMachine) {
        m.display_message("quarter ejected");
        m.set_state(GumballMachineState::NoQuarter);
    }
    fn turn_crank(&self, m: &mut GumballMachine) {
        m.display_message("you turned crank");
        let mut rng = rand::thread_rng();
        let winner: u8 = rng.gen_range(1..=10);
        if winner == 1 {
            m.set_state(GumballMachineState::Winner);
        } else {
            m.set_state(GumballMachineState::GumballSold);
        }
    }
    fn dispense(&self, m: &mut GumballMachine) {
        m.display_message("no gumball dispensed");
    }
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized {
        Box::new(HasQuarterState {})
    }
}
impl GumballMachineAction for GumballSoldState {
    fn insert_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot insert quarter until gumball is dispensed");
    }
    fn eject_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot eject quarter as there is no quarter");
    }
    fn turn_crank(&self, m: &mut GumballMachine) {
        m.display_message("you cannot turn crank as it is already turned");
    }
    fn dispense(&self, m: &mut GumballMachine) {
        m.display_message("dispensing a gumball");
        m.release_ball();
        if m.count > 0 {
            m.set_state(GumballMachineState::NoQuarter);
        } else {
            m.display_message("all gumballs were sold out");
            m.set_state(GumballMachineState::OutOfGumballs);
        }
    }
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized {
        Box::new(GumballSoldState {})
    }
}
impl GumballMachineAction for OutOfGumballsState {
    fn insert_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot insert quarter as machine has no gummball");
    }
    fn eject_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot eject quarter as machine has no gummball");
    }
    fn turn_crank(&self, m: &mut GumballMachine) {
        m.display_message("you cannot turn crank as machine has no gummball");
    }
    fn dispense(&self, m: &mut GumballMachine) {
        m.display_message("no gumball dispensed");
    }
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized {
        Box::new(OutOfGumballsState {})
    }
}
// Some changes from GumballSoldState for dispense().
impl GumballMachineAction for WinnerState {
    fn insert_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot insert quarter until gumball is dispensed");
    }
    fn eject_quarter(&self, m: &mut GumballMachine) {
        m.display_message("you cannot eject quarter as there is no quarter");
    }
    fn turn_crank(&self, m: &mut GumballMachine) {
        m.display_message("you cannot turn crank as it is already turned");
    }
    fn dispense(&self, m: &mut GumballMachine) {
        m.display_message("dispensing a gumball");
        m.release_ball();
        if m.count > 0 {
            m.release_ball();
            m.display_message("YOU'RE A WINNER! You got two gumballs for your quarter");
        }
        if m.count > 0 {
            m.set_state(GumballMachineState::NoQuarter);
        } else {
            m.display_message("all gumballs were sold out");
            m.set_state(GumballMachineState::OutOfGumballs);
        }
    }
    fn new() -> Box<dyn GumballMachineAction> where Self: Sized {
        Box::new(WinnerState {})
    }
}

struct GumballMachine {
    /*
     * the number of gumballs in the machine.
     */
    count: i32,
    state: GumballMachineState
}

impl GumballMachine {
    /*
     * initial inventory of gumballs.
     */
    fn new(count: i32) -> Self {
        if count > 0 {
            GumballMachine { count: count, state: GumballMachineState::NoQuarter }
        } else {
            GumballMachine { count: count, state: GumballMachineState::OutOfGumballs }
        }
    }
    fn eject_quarter(&mut self) {
        from(self.state).eject_quarter(self);
    }
    fn insert_quarter(&mut self) {
        from(self.state).insert_quarter(self);
    }
    fn release_ball(&mut self) {
        if self.count > 0 {
            self.display_message("A gumball comes rolling out the slot...");
            self.count = self.count - 1;
        }
    }
    fn display_message(&self, msg: &str) {
        println!("{} says \"{}\"", self, msg);
    }
    fn set_state(&mut self, s: GumballMachineState) {
        self.state = s;
    }
    fn turn_crank(&mut self) {
        from(self.state).turn_crank(self);
        from(self.state).dispense(self);
    }
}

impl fmt::Display for GumballMachine {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Gumball ( count: {}, state: {} )", self.count, self.state)
	}
}

fn main() {
    let mut machine = GumballMachine::new(5);
    // NoQuarter -insert_quarter-> HasQuarter
    machine.insert_quarter();
    machine.insert_quarter();
    // HasQuarter -eject_quarter-> NoQuarter
    machine.eject_quarter();
    machine.eject_quarter();
    machine.insert_quarter();
    // HasQuarter -turn_crank-> GumballSold -dispense-> NoQuarter/OutOfGumballs
    machine.turn_crank();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
}
