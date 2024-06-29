#![feature(noop_waker)]
#![no_std]
#![no_main]

// uses panic handler. TODO: redundant when we can `use syterkit`.
use syterkit_100ask_d1_h as _;
// TODO: `use syterkit::main`.
use syterkit_macros::main;

#[main]
async fn main() {
    // TODO contents
}
