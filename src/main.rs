use std::{fs::{File, self}, io::{Read, Write}};
use std::process::Command;
fn main() {
    //shell commands 
    //nasm -felf64 test.asm && ld test.o && ./a.out

    let mut f = File::create("foo.txt").expect("foo this shiz");
    f.write_all(b"i am writing this as a test \nthis should be on a new lineeferfefefrerferff").expect("writing failed");
    println!("Hello, world!");
    let data = fs::read_to_string("./test.crz").expect("unable to open le file");
    
    
    
    //command to assemble 
    Command::new("nasm")
    .arg("-felf64")
    .arg("test.asm")
    .output()
    .expect("failed nasm");

    //commmand to link to libs 
    Command::new("ld")
    .arg("test.o")
    .output()
    .expect("failed linker");

    //run program
   // Command::new("./a.out").output().expect("failed to execute");



    println!("{}",data);
}
