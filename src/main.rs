// TODO get editline C interop working

use std::io;
#[macro_use]
extern crate nom;

//#[derive(Debug,PartialEq)]
pub struct NomAstT {
  pub tag:   String,
  pub contents: String,
    //mpc_state_t state,
  pub children_num: u8,
  pub children: Vec<NomAstT>,
}

fn main () {
  println!("Lispy Version 0.0.0.0.1");
  println!("Press Ctrl+c to Exit\n");

  let mut input = String::new();

  loop {
    print!("lispy> ");
    match io::stdin().read_line(&mut input) {
      Ok(n) => {
        println!("{} bytes read", n);
        println!("input {}", input);
      }
      Err(error) => println!("error: {}", error),
    }
  }
}
