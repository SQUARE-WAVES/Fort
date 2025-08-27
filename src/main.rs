mod vstack;
use vstack::*;
mod tokens;
use tokens::*;
mod dictionary;
use dictionary::*;
mod bifs;
mod vm;
use vm::*;

fn main() {
  let src = "1 (1 +) call .";
  let mut vm = VM::default();
  vm.eval(src);
}
