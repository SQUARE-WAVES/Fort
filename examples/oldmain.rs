use fort::{
  Fort,
  traits::TypeTag,
  Dict,
  Scope,
  V,
  Thread,
  parser::Repl,
  bifs
};

pub struct Basic {}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Nothing{}

impl TypeTag for Nothing {
  fn tag(&self) -> &'static str { "()" }
}

impl std::fmt::Display for Nothing {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    write!(f,"()")
  }
}

impl From<Nothing> for V<Basic> {
  fn from(n:Nothing) -> V<Basic> {
    V::Ext(n)
  }
}

fn new_nothing(th:&mut Thread<Basic>) -> Result<(),bifs::Error> {
  th.push(Nothing{});
  Ok(())
}

impl Fort for Basic {
  type Extension = Nothing;
  type Environment=();

  fn default_env() {}

  fn dictionary() -> Dict<Self> { 
    let mine = Scope::from([bifs::def("nothing"," --> ()",new_nothing)]);
    let root = bifs::built_ins().merge(mine);
    Dict::new(root)
  }
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
  run_repl()?;
  Ok(())
}

fn run_repl() -> Result<(),Box<dyn std::error::Error>> {
  let mut buff = String::new();
  let stdin = std::io::stdin();
  let mut dict = Basic::dictionary();
  let mut repl = Repl::new();
  let mut vm = Thread::new(&mut dict,());

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
