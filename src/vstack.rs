use crate::{V,F,VType};

#[derive(Default)]
pub struct Vstack {
  vs:Vec<V>
}

impl Vstack {
  pub fn push<T:Into<V>>(&mut self,val:T) {
    self.vs.push(val.into());
  }

  pub fn pop(&mut self) -> Result<V,StackErr> {
    self.vs.pop().ok_or(StackErr::Underflow)
  }

  pub fn peek(&self,n:usize) -> &[V] {
    let start = self.vs.len().saturating_sub(n);
    &self.vs[start..]
  }

  pub fn dropn(&mut self,n:usize) {
    let start = self.vs.len().saturating_sub(n);
    self.vs.drain(start..);
  }

  pub fn clear(&mut self) {
    self.vs.clear();
  }

  pub fn tpop<T:VType>(&mut self) -> Result<T,StackErr> {
    let v = self.vs.pop().ok_or(StackErr::Underflow)?;
    match v.try_into() {
      Ok(n) => Ok(n),
      Err(v) => {
        let err = StackErr::Type(T::type_tag(),v.type_tag());
        self.vs.push(v);
        Err(err)
      }
    }
  }

  pub fn len(&self) -> usize {
    self.vs.len()
  }

  pub fn lst(&mut self,start:usize) -> V {
    let list_vec : Vec<V> = self.vs.drain(start..).collect();
    V::L(list_vec.into())
  }

  pub fn def(&mut self,start:usize) -> F {
    let list_vec : Vec<V> = self.vs.drain(start..).collect();
    F::Def(list_vec.into())
  }
 
  pub fn print(&self) {
    if self.vs.is_empty() {
      print!("-=Empty=-");
      return;
    }

    print!("-=");
    for v in self.vs.iter() {
      print!(" {v} ");
    }
    print!("=-");
  }
}

#[derive(Debug,Copy,Clone)]
pub enum StackErr {
  Underflow,
  Type(&'static str,&'static str)
}

impl std::fmt::Display for StackErr {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Underflow => write!(f,"stack underflow"),
      Self::Type(exp,rcv) => write!(f,"incorrect type, expected: {exp}, got: {rcv}")
    }
  }
}

impl std::error::Error for StackErr{}
