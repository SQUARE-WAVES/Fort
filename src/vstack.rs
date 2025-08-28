use std::sync::Arc;
use crate::bifs::BifError;

#[derive(Debug,Clone)]
pub enum F {
  Bif(fn(&mut Vstack) -> Result<(),BifError>),
  Def(Arc<[V]>)
}

impl PartialEq for F {
  fn eq(&self,other:&F) -> bool {
    match (self,other) {
      (Self::Bif(n), Self::Bif(m)) => std::ptr::fn_addr_eq(*n,*m),
      (Self::Def(a), Self::Def(b)) => Arc::ptr_eq(a,b),
      _ => false
    }
  }
}

impl F {
  pub fn run(&self,vs:&mut Vstack) -> Result<(),BifError> {
    match self {
      Self::Bif(f) => f(vs),

      Self::Def(arc) => {
        for v in arc.iter() {
          match v {
            V::C(f) => {
              f.run(vs)?;
            }
            x => {
              vs.push(x.clone())
            }
          };
        }

        Ok(())
      }
    }
  }
}

#[derive(Debug,Clone)]
pub enum V {
  Z(f64),
  L(Arc<[V]>),
  I(i64),
  F(F),
  C(F),
  B(bool)
}

#[derive(Default)]
pub struct Vstack {
  vs:Vec<V>
}

impl Vstack {
  pub fn push(&mut self,val:V) {
    self.vs.push(val);
  }

  pub fn pop(&mut self) -> Option<V> {
    self.vs.pop()
  }

  /* not using these yet
  pub fn popz(&mut self) -> Option<f64> {
    match self.vs.pop() {
      Some(V::Z(v)) => Some(v),
      _ => None
    }
  }

  pub fn popi(&mut self) -> Option<i64> {
    match self.vs.pop() {
      Some(V::I(v)) => Some(v),
      _ => None
    }
  }*/

  pub fn popl(&mut self) -> Option<Arc<[V]>> {
    match self.vs.pop() {
      Some(V::L(v)) => Some(v),
      _ => None
    }
  }

  pub fn popb(&mut self) -> Option<bool> {
    match self.vs.pop() {
      Some(V::B(v)) => Some(v),
      _ => None
    }
  }

  pub fn popf(&mut self) -> Option<F> {
    match self.vs.pop() {
      Some(V::F(f)) => Some(f),
      _ => None
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

  /*
  pub fn is_empty() -> bool {
    self.vs.is_empty()
  }

  pub fn frame(&mut self,amt:usize) -> Vstack {
    Vstack{ vs:self.vs.drain((self.len()-amt)..).collect() }
  }
  */

  pub fn print(&self) {
    println!("{:?}",self.vs);
  }
}
