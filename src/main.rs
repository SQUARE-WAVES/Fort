mod vstack;
use vstack::*;
mod tokens;
use tokens::*;
mod dictionary;
use dictionary::*;
mod bifs;
mod lazy;
mod vm;
use vm::VM;

fn main() -> Result<(),Box<dyn std::error::Error>> {
  let src = "0 (dup 10 == ( ) (1 + dup) if .) coal";
  let mut vm = VM::default();
  
  vm.eval(src)?; 
  Ok(())
}
