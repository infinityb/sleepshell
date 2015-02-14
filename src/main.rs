#![feature(std_misc, core, libc, io, box_syntax, plugin)]
#![plugin(phf_macros)]

extern crate libc;
extern crate time;
extern crate phf;

use std::old_io::timer::sleep;
use std::time::duration::Duration;

mod holidays;

fn getppid() -> i32 {
    unsafe { ::libc::funcs::posix88::unistd::getppid() }
}

trait Event {
    fn activate(&mut self, &::time::Tm);
}

fn main() {
    let original_parent = getppid();

    let mut announced: Vec<Box<Event>> = Vec::new();
    holidays::register_all(&mut announced);

    while original_parent == getppid() {
        let now = time::now_utc();
        for event in announced.iter_mut() {
            event.activate(&now);
        }

        sleep(Duration::seconds(60));
    }
}

