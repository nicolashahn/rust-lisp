// TODO get editline C interop working

use std::io;
use std::io::prelude::*;
use std::str;
#[macro_use]
extern crate nom;

//#[derive(Debug,PartialEq)]
pub struct NomAstT {
    pub tag: String,
    pub contents: String,
    //mpc_state_t state,
    pub children_num: u8,
    pub children: Vec<NomAstT>,
}

named!(token<&str, Vec<&str>>,
       ws!(many0!(alt!(tag_s!("b") | tag_s!("a"))))
);

named!(s_expr<&str,Vec<&str>>,
       delimited!(tag_s!("("),
                      token,
                  tag_s!(")"))
);

fn main() {
    println!("Lispy Version 0.0.0.0.1");
    println!("Press Ctrl+c to Exit\n");

    loop {
        print!("lispy> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                let res = s_expr(&input);
                println!("response {:?}", res);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
