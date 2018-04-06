//! Core `StateMachine` logic
//! Handle the state and parameters of the machine, with the logic to process actions

use std::fmt;

use reducer::Reducer;
use state::State;
use action::Action;

/// State machine with a state of type `T`
pub struct StateMachine<'a, T>
where
    T: 'a + Copy + Clone,
{
    reducers: Vec<Box<&'a Reducer<T>>>,
    state: State<T>,
}

impl<'a, T> StateMachine<'a, T>
where
    T: 'a + Copy + Clone,
{
    /// Create a new state machine from an initial state
    /// Note: The returned machine will be empty, so actions won't have any
    /// effects until reducers are added.
    pub fn new(initial_state: T) -> StateMachine<'a, T> {
        StateMachine {
            reducers: Vec::new(),
            state: State::new(&initial_state),
        }
    }

    /// Add a reducer to the state machine
    pub fn push_reducer(&mut self, reducer: &'a Reducer<T>) -> &mut Self {
        self.reducers.push(Box::new(reducer));
        self
    }

    /// Action processing logic, generate the new state from (prev_state, action) and
    /// set the internal state to the new state.
    pub fn process(&self, a: &Action) -> &Self {
        let mut new_state = self.state.get_state();
        for reducer in &self.reducers {
            new_state = reducer.process(new_state, a);
        }
        self.state.set_state(&new_state);
        self
    }

    /// Recover a copy of the state
    /// # Warning
    /// This method will clone the entire state, and is therefore potentially expensive
    pub fn get_state(&self) -> T {
        self.state.get_state()
    }
}

impl<'a, T> fmt::Debug for StateMachine<'a, T>
where
    T: 'a + Copy + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateMachine {{ reducers: {:?} }}", self.reducers.len())
    }
}

impl<'a, T> fmt::Display for StateMachine<'a, T>
where
    T: 'a + Copy + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateMachine")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use action::Action;

    struct IncrementReducer {}

    impl Reducer<u32> for IncrementReducer {
        fn process(&self, state: u32, action: &Action) -> u32 {
            match action.cat {
                "increment" => state + 1,
                _ => state,
            }
        }
    }

    #[test]
    fn can_get_state() {
        let sm = StateMachine::new(0);
        assert_eq!(sm.get_state(), 0);
    }

    #[test]
    fn reducer_works() {
        let mut sm = StateMachine::new(0);

        sm.push_reducer(&IncrementReducer {});

        assert_eq!(sm.get_state(), 0);
        sm.process(&Action::new("increment", &[]));
        assert_eq!(sm.get_state(), 1);
    }

}
