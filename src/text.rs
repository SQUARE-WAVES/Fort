use std::{
  ops::Deref,
  sync::Arc,
};

#[derive(Debug,Clone,PartialEq)]
pub struct Txt(Arc<str>);


impl std::fmt::Display for Txt {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    let Txt(arc) = self;
    write!(f,"{arc}")
  }
}

impl From<&str> for Txt {
  fn from(s:&str) -> Self {
    Txt(s.into())
  }
}

impl Deref for Txt {
  type Target=str;
  fn deref(&self) -> &str {
    let Txt(arc) = self;
    arc
  }
}


#[derive(Debug,Clone,PartialEq)]
pub struct Sym(Arc<str>);


impl std::fmt::Display for Sym {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    let Sym(arc) = self;
    write!(f,"<[{arc}]>")
  }
}

impl From<&str> for Sym {
  fn from(s:&str) -> Self {
    Sym(s.into())
  }
}

impl Deref for Sym {
  type Target=str;
  fn deref(&self) -> &str {
    let Sym(arc) = self;
    arc
  }
}
