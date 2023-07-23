extern crate linux_errnos;

use linux_errnos::*;

pub fn main() {
    for errno in Errno::iter() {
        println!(
            "{}: {}",
            errno.name().unwrap(),
            errno.description().unwrap()
        );
    }
}
