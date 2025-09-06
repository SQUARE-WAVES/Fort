use fort::{
  Fort,
  TypeTag,
  TaggedType,
  Dict,
  Scope,
  V,
  Thread,
  parser::Repl,
  bifs
};

//You have to write a bit of boilerplate to make a simple Fort
//it's annoying and maybe later I can make somethign to help
//but the point of this is that normally each of these bits of code 
//would be important to making the language work for your purposes 

//----------------------------------------------------------------
//this is a type to use as our extension type
//I'm not sure if there is a good way to make it so you can
//specify an extension but you don't have to.
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Nothing{}

//it needs to implement these 3 traits in order to play nicely
//with the rest of the stuff
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

//you don't have to do these two, but they mke writing bifs a lot easier
//as you can use tpop and the automatic type converting features of the
//stack, it basically saves you a bunch of "if let V::Ext(your thing) = v {"
//type stuff
impl TryFrom<V<Basic>> for Nothing {
  type Error = V<Basic>;
  fn try_from(v:V<Basic>) -> Result<Self,Self::Error> {
    if let V::Ext(n) = v {
      Ok(n)
    }
    else {
      Err(v)
    }
  }
}

impl TaggedType for Nothing {
  fn type_tag() -> &'static str {
    "nothing"
  }
}


//here are some of our BIFs, these have to be implemented as static functions for now.
//I'm going to play around with making them work as closures and structs, but probably
//static functions will be the best way in general to do it.

//here is a built in function that lets us create a nothing
fn new_nothing(th:&mut Thread<Basic>) -> Result<(),bifs::Error> {
  th.push(Nothing{});
  Ok(())
}

//here is a nother one that lets us "use" nothings
fn nothing_burger(th:&mut Thread<Basic>) -> Result<(),bifs::Error> {
  let _n : Nothing = bifs::tpop(th,"bottom bread")?;
  let v = bifs::param(th,"the meat")?;
  let _n2 : Nothing = bifs::tpop(th,"top bread")?;

  println!("wow {v} is a real nothing burger");
  Ok(())
}


//now we actually make our Fort type, it mostly
//just needs to specifiy the types and how to make the 
//root environment (in this case no environment)
//and basic dictionaries.
pub struct Basic {}

impl Fort for Basic {
  type Extension = Nothing;
  type Environment=();

  fn default_env() {}

  fn dictionary() -> Dict<Self> { 
    
    let mine = Scope::from([
      bifs::def("nothing"," --> ()",new_nothing),
      bifs::def("nothingburger", "() ? () --> ",nothing_burger)
    ]);

    let root = bifs::built_ins().merge(mine);
    Dict::new(root)
  }
}

//wow we did it, now we can run a repl
fn main() -> Result<(),Box<dyn std::error::Error>> {
  let mut buff = String::new();
  let stdin = std::io::stdin();
  let mut dict = Basic::dictionary();
  let mut repl = Repl::new();
  let mut vm = Thread::new(&mut dict,());
  println!("LETS GOOOOOO!");
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
