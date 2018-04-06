//! Reducers
//! Stateless functions used by the state machine to process actions.
//! Each action will be fed to a reducer if it is
//! binded with the `StateMachine` that received this action.

use action::Action;

/// core logic trait, any structure that implement this trait
/// can be used to manipulate state.
pub trait Reducer<T> {
    /// Take the ownership of the state, and return the state after application of the action
    fn process(&self, state: T, action: &Action) -> T;
}

impl<T> From<fn(T, &Action) -> T> for PureReducer<T> {
    /// Create a pure reducer from a compatible function
    fn from(f: fn(T, &Action) -> T) -> PureReducer<T> {
        PureReducer { f }
    }
}

/// Wrapper to use a pure function as a reducer
struct PureReducer<T> {
    /// Function to be called when reducing
    f: fn(T, &Action) -> T,
}

impl<T> Reducer<T> for PureReducer<T> {
    /// Call the reducer function
    fn process(&self, state: T, action: &Action) -> T {
        (self.f)(state, action)
    }
}

/// `LoggingReducer`
/// Print the category of each action received
/// Leave the state unchanged
#[derive(Debug)]
pub struct LoggingReducer {}

impl<T> Reducer<T> for LoggingReducer {
    /// Print the action and give back state ownership
    fn process(&self, state: T, action: &Action) -> T {
        println!("Processing {:?}", action);
        state
    }
}
