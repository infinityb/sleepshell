#![feature(std_misc, core, libc, io, box_syntax)]

extern crate libc;
extern crate time;

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
    announced.push(box holidays::Valentines::new());

    while original_parent == getppid() {
        sleep(Duration::seconds(60));

        let tm = time::now();
        for event in announced.iter_mut() {
            event.activate(&tm);
        }
    }
}