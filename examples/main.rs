//! Example application
//! It show reducers flow and how to implement it

extern crate ralf;

use ralf::*;

/// Simple reducer incrementing a u32 state only if the action category is "increment"
/// # Examples
/// ```rust
/// let reducer = IncrementReducer {};
/// assert_eq!(reducer.process(0, &Action::new("increment", &[])), 1);
/// ```
struct IncrementReducer {}

impl Reducer<u32> for IncrementReducer {
    fn process(&self, state: u32, action: &Action) -> u32 {
        match action.cat {
            "increment" => state + 1,
            _ => state,
        }
    }

    fn id(&self) -> &str { "increment" }
}


/// Minimal example on the reducer flow
/// This example does:
/// - Create a state machine with an initial state of 0
/// - bind the reducers to the state machine, one will increment the state
///   and the other will display the final state value
/// - Pass an invalid key to the reducer, ensuring the state wasn't changed
/// - Pas a valid key to the reducer, ensuring the state was changed

fn main() {
    let mut sm = StateMachine::new(0);

    sm.push_reducer(Box::new(IncrementReducer {}))
        .push_reducer(Box::new(LoggingReducer::new(
            // Watching for the "increment" id (defined in the implementation above)
            "increment"
        )));

    sm.process(&Action::new("invalid", &[]));
    assert_eq!(sm.get_state(), 0);

    sm.process(&Action::new("increment", &[]));
    assert_eq!(sm.get_state(), 1);
}
