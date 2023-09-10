use std::{fs::{File, self}, io::{Read, Write}, option};
use std::process::Command;

#[derive(Debug, Clone)]
enum Binop{
    Add(Expr,Expr)
}

#[derive(Debug, Clone)]
enum Expr{
    Int(i32),
    Bin(Box<Binop>)
}

impl Expr{
    fn print(&self){
        match self {
            Expr::Int(i) => print!("{}",i),
            Expr::Bin(Bi) => {
                match &**Bi {
                    Binop::Add(lhs, rhs) => {
                        print!("(");
                        lhs.print();
                        print!("+");
                        rhs.print();
                        print!(")")                    },
                }
            },
        }
    }
}

//Token::Tint(val) =>Expr::Int(val)

fn buildTree(tokens:Vec<Token>) -> Option<Expr>{
    let mut left:Option<Expr> = None;
    let mut stack = Vec::new();
    let mut itter = tokens.iter();

    while let Some(token) = itter.next(){
        match token {
            Token::Tlet => todo!(),
            Token::Texit => todo!(),
            Token::Tadd =>{
               match left {
                Some(Ex) => {
                    match itter.next(){
                        Some(Te) => match Te {
                            Token::Tint(v)=>left = {
                                Some(Expr::Bin(Box::new(Binop::Add(Ex, Expr::Int(*v)))))
                            },

                            _ => panic!("invalid syntax"),
                        }
                        ,
                        None => panic!("token expected"),
                    }
                },
                None => panic!("need to have int before add"),
            }
            },
            Token::Tint(val) =>{
                match left {
                    Some(Ex) => panic!("two expr in a row"),
                    None => left = Some(Expr::Int(*val)),
                }
                stack.push(Token::Tint(*val));
            },
            Token::Tsemi => todo!(),
            Token::Tindent => todo!(),
            Token::Tassign => todo!(),
        } 
    }




   
 
    return left;
    }
    



#[derive(Debug)]
enum Token{
    Tlet,
    Texit,
    Tadd,
    Tint(i32),
    Tsemi,
    Tindent,
    Tassign
}


fn main() {

    
    //read from input file
    
    let data = fs::read_to_string("./test.crz").expect("unable to open le file");
    
    



    let mut f = File::create("bar.asm").expect("asm file creation failed");
    
    println!("{}",data);
    let tokens = tokenize(data);
    println!("{:?}", tokens);
    

    buildTree(tokens).unwrap().print();
    let mut output = String::new();
    //buildstr(&mut output);
   // buils_asm(&mut output, tokens);
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



    
}

fn buils_asm(str: &mut String, tokens: Vec<Token>){
    str.push_str("global _start\n\n");
    str.push_str("section .text\n\n");
    str.push_str("_start:\n");

    for token in tokens{
       match token {
        Token::Tlet => println!("not yet implmented"),
        Token::Texit => {
            str.push_str(" mov rax, 60\n");
            str.push_str(" mov rdi, 42\n");
            str.push_str(" syscall \n");
        },
        Token::Tadd => println!("not yet implmented"),
        Token::Tint(_) => println!("not yet implmented"),
        Token::Tsemi => println!("not yet implmented"),
        Token::Tindent => println!("not yet implmented"),
        Token::Tassign => println!("not yet implmented"),
    }
    }



}


fn isvalididentifer(str:&String)-> bool{
    if str.len() == 0 {
        return false;
    }
    for(i,c) in str.chars().enumerate(){
        if((i == 0 && !c.is_alphabetic() )|| !c.is_alphanumeric()) {
            return false;
        }
    }
    return true;
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

fn tokenize (str: String)-> Vec<Token>{
    let mut res = Vec::new();
    let mut buff = String::new();
    let mut e = str.chars();
    loop{
        match e.next() {
            Some(v) => {
                if v != ' ' && v != '\n' &&v != ';'{
                    buff.push(v);
                }else{
                   println!("|{}|",buff);
                    
                    if buff == "let"{
                        res.push(Token::Tlet)
                    }else if buff == "+" {
                        res.push(Token::Tadd);
                    }else if buff == "exit" {
                        res.push(Token::Texit);
                    }else if buff.parse::<i32>().is_ok() {
                        res.push(Token::Tint(buff.parse::<i32>().unwrap()));
                    }else if buff == "="{
                        res.push(Token::Tassign)
                    }else if  isvalididentifer(&buff) {
                        res.push(Token::Tindent)
                    }
                    if v == ';'{
                        res.push(Token::Tsemi)
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