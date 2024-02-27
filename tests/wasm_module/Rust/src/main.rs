mod bindings;
use bindings::*;

extern crate alloc;

fn main() {
    println!("{}", psh::profiling::system::os_version().unwrap()); 
}