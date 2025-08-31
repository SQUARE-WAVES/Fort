mod values;
use values::*;
mod vstack;
use vstack::*;
mod tokens;
mod dictionary;
use dictionary::*;

mod bifs;
mod vm;
use vm::VM;

fn main() -> Result<(),Box<dyn std::error::Error>> {
  repl();
  Ok(())
}

fn repl() {
  let mut buff = String::new();
  let stdin = std::io::stdin();
  let mut vm = VM::default();

  loop {
    if stdin.read_line(&mut buff).is_err() {
      println!("bye");
      return
    }

    match vm.repl_buff(&mut buff) {
      Ok(()) => (),
      Err(e) => println!("err {e}")
    };
  }
}
