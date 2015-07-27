#![crate_name="rusty_cc"]

#![feature(test)]
#![allow(dead_code)]

#[macro_use]
mod debug;

mod constraint;
mod graph;
mod snapshot_vec;
mod cc;
mod unify;
mod bitvec;
