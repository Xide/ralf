//! `Ralf`
//! Distributed state machine with raft

#![deny(missing_docs, missing_debug_implementations, trivial_casts, trivial_numeric_casts,
        unsafe_code, unused_import_braces, unused_qualifications)]

mod state;
pub use state::State;

mod action;
pub use action::Action;

mod reducer;
pub use reducer::{Reducer, PureReducer, LoggingReducer};

mod state_machine;
pub use state_machine::StateMachine;

//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
