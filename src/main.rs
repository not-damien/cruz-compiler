use std::{fs::{File, self}, io::{Read, Write}};
use std::process::Command;

#[derive(Debug)]
enum Token{
    Tlet,
    Tadd,
    Tint,
    Tsemi
}


fn main() {

    
    //read from input file
    
    let data = fs::read_to_string("./test.crz").expect("unable to open le file");
    
    



    let mut f = File::create("bar.asm").expect("asm file creation failed");

    let tokens = parse(&data);
    println!("{:?}", tokens);
    
    let mut output = String::new();
    buildstr(&mut output);
    f.write_all(output.as_bytes()).expect("writing failed");



    //command to assemble 
    Command::new("nasm")
    .arg("-felf64")
    .arg("bar.asm")
    .output()
    .expect("failed nasm");

    //commmand to link to libs 
    Command::new("ld")
    .arg("bar.o")
    .output()
    .expect("failed linker");

    //run program
   // Command::new("./a.out").output().expect("failed to execute");



    println!("{}",data);
}



fn buildstr (str: &mut String){
    str.push_str("global _start\n\n");
    str.push_str("section .text\n\n");
    str.push_str("_start:\n");
    str.push_str(" mov rax, 1\n");
    str.push_str(" mov rdi, 1 \n");
    str.push_str(" mov rsi, msg\n");
    str.push_str(" mov rdx, msglen\n");
    str.push_str(" syscall \n");
    str.push_str(" mov rax, 60\n");
    str.push_str(" mov rdi, 0\n");
    str.push_str(" syscall \n");

    str.push_str("section .rodata\n\
    msg: db \"Hello, compiler!\", 10\n\
    msglen: equ $ - msg");

}


fn parse (str: &String)-> Vec<Token>{
    let mut res = Vec::new();


    let mut buff = String::new();
    let mut e = str.chars();
    loop{
        match e.next() {
            Some(v) => {
                if v != ' ' && v != '\n'{
                    buff.push(v);
                    
                }else{
                    println!("|{}|",buff);
                    if buff == "let"{
                        res.push(Token::Tlet)
                    }else if buff == "+" {
                        res.push(Token::Tadd);
                    }else if buff == ";" {
                        res.push(Token::Tsemi);
                    }else if buff.parse::<u32>().is_ok() {
                        res.push(Token::Tint);
                    }
                    buff.clear();
                    continue
                }
                //println!("{}",v)
            },
            None => break
        }
    }
    res
}