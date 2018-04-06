# Ralf

Distributed state machine library, written in Rust on top of [raft-rs](https://github.com/pingcap/raft-rs).

[![Build Status](https://travis-ci.org/Xide/ralf.svg?branch=master)](https://travis-ci.org/Xide/ralf)
[![Coverage Status](https://coveralls.io/repos/github/Xide/ralf/badge.svg?branch=master)](https://coveralls.io/github/Xide/ralf?branch=master)

[Crate documentation](https://xide.github.io/ralf)

## Examples

| Example                            | Description                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------------- |
| [Reducer flow](./examples/main.rs) | Create a simple local state machine, and see how to create reducers to handle events. |
