use std::sync::Arc;
use crate::{
  V,
  Txt,
  F,
  Thread,
  traits::{
    Fort,
    TypeTag,
    StackType
  }
};

mod stack;
mod math;
mod functional;
mod util;

pub struct Bif<S:Fort>(fn(&mut Thread<S>) -> Result<(),Error>);

impl<S:Fort> Bif<S> {
  pub fn call(&self,th:&mut Thread<S>) -> Result<(),Error> {
    (self.0)(th)
  }
}

impl<S:Fort> Clone for Bif<S> {
  fn clone(&self) -> Bif<S> {
    Bif(self.0)
  }
}

impl<S:Fort> From<fn(&mut Thread<S>) -> Result<(),Error>> for Bif<S> {
  fn from(f:fn(&mut Thread<S>) -> Result<(),Error>) -> Self {
    Self(f)
  }
}

impl<S:Fort> PartialEq for Bif<S> {
  fn eq(&self,o:&Self) -> bool {
    let(Self(n),Self(m)) = (self,o);
    std::ptr::fn_addr_eq(*n,*m)
  }
}

pub fn built_ins<S:Fort>() -> std::collections::HashMap<Arc<str>,F<S>> {
  std::collections::HashMap::from([
    def("dup","a --> a a",stack::dup),
    def("clear","as.. --> Empty",stack::clear),
    def("swap","a b --> b a",stack::swap),
    def("rot","a b c --> c a b",stack::rot),
    def("over","a b --> a b a",stack::over),
    def("drop","a b --> a",stack::drop),

    def("+","n1(I/Z) n2(I/Z) -> (n1+n2)(I/Z)",math::add),
    def("-","n1(I/Z) n2(I/Z) -> (n1-n2)(I/Z)",math::sub),
    def("*","n1(I/Z) n2(I/Z) -> (n1*n2)(I/Z)",math::mul),
    def("/","n1(I/Z) n2(I/Z) -> (n1/n2)(I/Z)",math::div),
    def("==","a b --> (a == b)(B)",math::eq),
    def("!=","a b --> (a != b)(B)",math::neq),

    def("map","[as...] proc(a --> b) --> [(a b)! ...]",functional::map),
    def("!","a(Fn ? -> ?) --> (whatever that fn does)",functional::call),
    
    def("if","ifTrue(fn) ifFalse(fn) c(B) --> (one of the 2 functions)",functional::cond),
    def("while","body (fn --> whatever) test(fn ? --> bool)",functional::while_loop),

    def(".","prints the current state of the thread",util::print),
    def("doc","prints a bif's documentation",util::doc),
    def("do_file","loads a file in a spare thread then applys it",util::do_file),
    def("I","Z --> I(rounded)",math::to_int),
    def("Z","I --> Z",math::to_float),
  ])
}

#[derive(Debug,Copy,Clone)]
pub enum Error {
  Underflow(&'static str),
  PType(&'static str,&'static str,&'static str),
  Internal(&'static str)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Error::Underflow(name) => write!(f,"stack underflow fetching param {name}"),
      Error::PType(nm,w,g) => write!(f,"wrong type for param {nm} wanted: {w} got: {g}"),
      Error::Internal(msg) => write!(f,"internal error {msg}")
    }
  }
}

impl std::error::Error for Error{}

//------------------------------------------------------------------------------------------------
//Utilities for writing BIFs 
pub fn tpop<S,Val>(th:&mut Thread<S>,name:&'static str) -> Result<Val,Error> 
where 
  S:Fort,
  Val:StackType<V<S>>
{
  let res = th.pop::<Val>().ok_or(Error::Underflow(name))?;
  match res {
    Ok(p) => Ok(p),
    Err(v) => {
      let out = Error::PType(name,Val::type_tag(),v.tag());
      th.push(v);
      Err(out)
    }
  }
}

pub fn param<S:Fort>(th:&mut Thread<S>,name:&'static str) -> Result<V<S>,Error> {
  th.popv().ok_or(Error::Underflow(name))
}

type BifType<S> = fn(&mut Thread<S>) -> Result<(),Error>;

pub fn def<S:Fort>(nm:&'static str,doc:&'static str,f:BifType<S>) -> (Arc<str>,F<S>) {
  let bif = F::Bif(nm,doc,f.into());
  (nm.into(),bif)
}
