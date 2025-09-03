use std::sync::Arc;
use crate::{
  V,
  ExtType,
  TypeTag,
  Txt,
  //Sym,
  VType,
  F,
  Thread,
  Vstack
};

mod stack;
mod math;
mod functional;
mod util;

pub fn built_ins<E:ExtType>() -> std::collections::HashMap<Arc<str>,F<E>> {
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
pub fn tpop<E:ExtType,Val:VType<E>>(stk:&mut Vstack<V<E>>,name:&'static str) -> Result<Val,Error> {
  let res = stk.pop::<Val>().ok_or(Error::Underflow(name))?;
  match res {
    Ok(p) => Ok(p),
    Err(v) => {
      let out = Error::PType(name,Val::type_tag(),v.tag());
      stk.push(v);
      Err(out)
    }
  }
}

pub fn param<E:ExtType>(stk:&mut Vstack<V<E>>,name:&'static str) -> Result<V<E>,Error> {
  stk.popv().ok_or(Error::Underflow(name))
}

pub fn def<E:ExtType>(nm:&'static str,doc:&'static str,f:fn(&mut Thread<E>)->Result<(),Error>) -> (Arc<str>,F<E>) {
  let bif = F::Bif(nm,doc,f.into());
  (nm.into(),bif)
}
