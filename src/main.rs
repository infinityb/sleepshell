#![feature(std_misc, libc, io)]

extern crate libc;

use std::old_io::timer::sleep;
use std::time::duration::Duration;
use libc::funcs::posix88::unistd::getppid;

fn main() {
    let original_parent = unsafe { getppid() };
    while original_parent == unsafe { getppid() } {
        sleep(Duration::seconds(60));
    }
}
