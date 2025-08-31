use crate::{V,F,Vstack,StackErr};

mod stack;
mod math;
mod functional;
mod util;

pub fn root_dict() -> std::collections::HashMap<String,F> {
  std::collections::HashMap::from([
    ("dup".into(),F::Bif(stack::dup)),
    ("clear".into(),F::Bif(stack::clear)),
    ("swap".into(),F::Bif(stack::swap)),
    ("rot".into(),F::Bif(stack::rot)),
    ("over".into(),F::Bif(stack::over)),
    ("drop".into(),F::Bif(stack::drop)),

    ("+".into(),F::Bif(math::add)),
    ("-".into(),F::Bif(math::sub)),
    ("*".into(),F::Bif(math::mul)),
    ("/".into(),F::Bif(math::div)),
    (".".into(),F::Bif(util::print_stack)),
    ("==".into(),F::Bif(math::eq)),
    ("!=".into(),F::Bif(math::neq)),

    ("map".into(),F::Bif(functional::map)),
    ("call".into(),F::Bif(functional::call)),
    
    ("if".into(),F::Bif(functional::cond)),
    ("while".into(),F::Bif(functional::while_loop))
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
