use std::sync::Arc;

#[derive(Debug,Clone)]
pub enum F {
  Bif(fn(&mut Vstack)),
  Def(Arc<[V]>)
}

impl F {
  pub fn run(&self,vs:&mut Vstack) {
    match self {
      Self::Bif(f) => f(vs),
      Self::Def(arc) => {
        for v in arc.iter() {
          match v {
            V::C(f) => {
              f.run(vs)
            }
            x => {
              vs.push(x.clone())
            }
          }
        }
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

  pub fn popz(&mut self) -> Option<f64> {
    match self.vs.pop() {
      Some(V::Z(v)) => Some(v),
      _ => None
    }
  }

  pub fn popl(&mut self) -> Option<Arc<[V]>> {
    match self.vs.pop() {
      Some(V::L(v)) => Some(v),
      _ => None
    }
  }

  pub fn popi(&mut self) -> Option<i64> {
    match self.vs.pop() {
      Some(V::I(v)) => Some(v),
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

  pub fn print(&self) {
    println!("{:?}",self.vs);
  }
}
