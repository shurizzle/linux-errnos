extern crate linux_errno;

use linux_errno::*;

pub fn main() {
    for errno in Errno::iter() {
        println!(
            "{}: {}",
            errno.name().unwrap(),
            errno.description().unwrap()
        );
    }
}
