//! Action are data type containing parameters for the state machine transition
//! They are the events sent to the reducers

use std::fmt;

/// `Action` data type, contain a string identifier and raw bytes
pub struct Action<'a, 'b> {
    /// Category of the action, simmilar to Redux action type (reserved keywork in Rust)
    pub cat: &'a str,
    /// Raw data associated with the action, parsing is left to the reducer
    /// Might change in the future
    pub data: &'b [u8],
}

impl<'a, 'b> Action<'a, 'b> {
    /// Create an action from it's category and raw data
    pub fn new(cat: &'a str, data: &'b [u8]) -> Action<'a, 'b> {
        Action { cat, data }
    }
}

impl<'a, 'b> fmt::Debug for Action<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Action {{ cat: {}, bytes_count: {} }}",
            self.cat,
            self.data.len()
        )
    }
}

impl<'a, 'b> fmt::Display for Action<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "action({})", self.cat)
    }
}
