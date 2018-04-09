//! Reducers
//! Stateless functions used by the state machine to process actions.
//! Each action will be fed to a reducer if it is
//! binded with the `StateMachine` that received this action.

use std::fmt;
use action::Action;

/// core logic trait, any structure that implement this trait
/// can be used to manipulate state.
pub trait Reducer<T> {
    /// Take the ownership of the state, and return the state after application of the action
    fn process(&self, state: T, action: &Action) -> T;

    /// id of the reducer.
    /// Reducers with the same id will share their state
    ///
    /// # Warning
    /// be careful with id conflict as there is no way to ensure which
    /// reducer will run first for this id (it currently depends on the order
    /// they were inserted in the state machine)
    fn id(&self) -> &str;

}

/// Wrapper to use a pure function as a reducer
pub struct PureReducer<'a, T> {
    /// id of the reducer
    id: &'a str,

    /// Function to be called when reducing
    f: fn(T, &Action) -> T,
}

impl<'a, T> PureReducer<'a, T> {
    /// Create a new pure reducer from it's id and function
    pub fn new(id: &'a str, f: fn(T, &Action) -> T) -> PureReducer<'a, T> {
        PureReducer {
            id,
            f
        }
    }
}

impl<'a, T> Reducer<T> for PureReducer<'a, T> {
    /// Getter to the structure id
    fn id(&self) -> &str { self.id }

    /// Call the reducer function
    fn process(&self, state: T, action: &Action) -> T {
        (self.f)(state, action)
    }
}

impl<'a, T> fmt::Debug for PureReducer<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PureReducer {{ id: {} }}",
            self.id,
        )
    }
}

/// `LoggingReducer`
/// Print the category of each action received by the reducer `id`
/// Leave the state unchanged
#[derive(Debug)]
pub struct LoggingReducer<'a> {
    id: &'a str,
}

impl<'a, T> Reducer<T> for LoggingReducer<'a> {
    /// Getter to the structure id
    fn id(&self) -> &str { self.id }

    /// Print the action and give back state ownership
    fn process(&self, state: T, action: &Action) -> T {
        println!("Processing {:?}", action);
        state
    }
}


impl<'a> LoggingReducer<'a> {
    /// Create a new logging reducer watching for reducer `id`
    pub fn new(id: &'a str) -> LoggingReducer<'a> {
        LoggingReducer { id }
    }
}


#[cfg(test)]
mod tests {
    use super::PureReducer;
    use action::Action;
    use state_machine::StateMachine;

    fn pure_reducer_increment(x: u32, _a: &Action) -> u32 {
        x + 1
    }

    #[test]
    fn can_create_reducer_from_function() {
        let r : PureReducer<u32> = PureReducer::new("increment", pure_reducer_increment);

        let mut sm = StateMachine::new(0);
        sm.push_reducer(Box::new(r));

        assert_eq!(sm.get_state(), 0);
        sm.process(&Action::new("some_action", &[]));
        assert_eq!(sm.get_state(), 1);
    }

}
