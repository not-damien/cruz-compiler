use std::{fs::{File, self}, io::{Read, Write}};
fn main() {
    let mut f = File::create("foo.txt").expect("foo this shiz");
    f.write_all(b"i am writing this as a test \nthis should be on a new line").expect("writing failed");
    println!("Hello, world!");
    let data = fs::read_to_string("./test.crz").expect("unable to open le file");
    println!("{}",data);
}
