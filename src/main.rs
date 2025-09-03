mod text;
use text::*;
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
use vm::Thread;

fn main() -> Result<(),Box<dyn std::error::Error>> {
  run_repl()?;
  Ok(())
}

fn run_repl() -> Result<(),Box<dyn std::error::Error>> {
  let mut buff = String::new();
  let stdin = std::io::stdin();
  let mut dict = Dict::<u8>::new(bifs::built_ins::<u8>().into());
  let mut repl = parser::Repl::default();
  let mut vm = Thread::as_list(&mut dict);

  loop {
    if stdin.read_line(&mut buff).is_err() {
      println!("bye");
      return Ok(());
    }

    match repl.buff(&mut vm,&mut buff) {
      Ok(()) => (),
      Err(e) => println!("err {e}")
    };
  }
}

impl ExtType for u8 {}
impl TypeTag for u8 {
  fn tag(&self) -> &'static str { "byte" }
}
