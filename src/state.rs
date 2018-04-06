//! Wrapper around a generic state type that implement Clone
//! Allow immutable query and mutation ofthe underlying data
use std::cell::Cell;

/// `State` is a wrapper around your state, allowing querying and modification
#[derive(Debug)]
pub struct State<T>
where
    T: Copy + Clone,
{
    state: Cell<T>,
}

impl<T> State<T>
where
    T: Copy + Clone,
{
    /// Create a new state from an initial value
    pub fn new(initial_state: &T) -> State<T> {
        State::<T> {
            state: Cell::new(*initial_state),
        }
    }

    /// return a clone of the underlying data
    pub fn get_state(&self) -> T {
        self.state.get()
    }

    /// Edit the underlying data immmutably
    pub fn set_state(&self, new_state: &T) {
        self.state.set(*new_state)
    }
}
