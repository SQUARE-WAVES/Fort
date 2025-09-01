use std::sync::Arc;
use crate::{
  V,
  F,
  functions::BifPtr,
  Vstack,
  StackErr
};

mod stack;
mod math;
mod functional;
mod util;

fn def(nm:&'static str,doc:&'static str,f:BifPtr) -> (Arc<str>,F) {
  let bif = F::Bif(nm,doc,f);
  (nm.into(),bif)
}

pub fn root_dict() -> std::collections::HashMap<Arc<str>,F> {
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

    def("doc","prints a bif's documentation",util::doc)
  ])
}

#[derive(Debug,Copy,Clone)]
pub enum Error {
  Param(&'static str,StackErr)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    let Error::Param(pname,root) = self;
    write!(f,"parameter error on param:{pname}, cause:{root}")
  }
}

impl std::error::Error for Error{}

pub fn p_err(nm:&'static str) -> impl FnOnce(StackErr) -> Error {
  move|se|Error::Param(nm,se)
}

pub fn punder(nm:&'static str) -> Error {
  Error::Param(nm,StackErr::Underflow)
}

pub fn ptyp_err(nm:&'static str,exp:&'static str,rcv:&'static str) -> Error {
  Error::Param(nm,StackErr::Type(exp,rcv))
}
