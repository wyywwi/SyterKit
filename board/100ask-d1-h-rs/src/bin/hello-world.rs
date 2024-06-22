#![no_std]
#![no_main]

// uses panic handler. TODO: redundant when we can `use syterkit`.
use syterkit_100ask_d1_h as _;
// TODO: `use syterkit::entry`.
use syterkit_macros::entry;

#[entry]
async fn main() {
    // TODO contents
}
