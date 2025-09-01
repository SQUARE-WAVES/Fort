mod values;
use values::*;
mod vstack;
use vstack::*;
mod tokens;
mod dictionary;
use dictionary::*;
mod functions;
use functions::F;
mod parser;

mod bifs;
mod vm;
use vm::VM;

fn main() -> Result<(),Box<dyn std::error::Error>> {
  run_repl();
  Ok(())
}

fn run_repl() {
  let mut buff = String::new();
  let stdin = std::io::stdin();
  let mut vm = VM::default();
  let mut repl = parser::Repl::default();

  loop {
    if stdin.read_line(&mut buff).is_err() {
      println!("bye");
      return
    }

    match repl.buff(&mut vm,&mut buff) {
      Ok(()) => (),
      Err(e) => println!("err {e}")
    };
  }
}
