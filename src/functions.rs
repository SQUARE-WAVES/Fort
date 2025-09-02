use std::sync::Arc;

use crate::{
  V,
  Thread,
  bifs::Error as BifError
};

pub type BifPtr = fn(&mut Thread) -> Result<(),BifError>;

#[derive(Debug,Clone)]
pub enum F {
  Bif(&'static str,&'static str,BifPtr),
  Def(Arc<str>,Arc<[V]>),
  Anon(Arc<[V]>)
}

impl std::fmt::Display for F {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Bif(nm,_,_) => write!(f,"{nm}"),
      Self::Def(nm,_) => write!(f,"{nm}"),
      Self::Anon(_) => write!(f,"fn"),
    }
  }
}

impl PartialEq for F {
  fn eq(&self,other:&F) -> bool {
    match (self,other) {
      (Self::Bif(_,_,n), Self::Bif(_,_,m)) => std::ptr::fn_addr_eq(*n,*m),
      (Self::Def(_,a), Self::Def(_,b)) => Arc::ptr_eq(a,b),
      (Self::Anon(a), Self::Anon(b)) => Arc::ptr_eq(a,b),
      _ => false
    }
  }
}

impl F {
  pub fn run(&self,th:&mut Thread) -> Result<(),BifError> {
    match self {
      Self::Bif(_,_,f) => f(th),

      Self::Def(_,arc) => {
        for v in arc.iter() {
          match v {
            V::C(f) => {
              f.run(th)?;
            }
            x => {
              th.push_val(x.clone());
            }
          };
        }

        Ok(())
      },

      Self::Anon(arc) => {
        for v in arc.iter() {
          match v {
            V::C(f) => {
              f.run(th)?;
            }
            x => {
              th.push_val(x.clone())
            }
          };
        }

        Ok(())
      }
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Self::Bif(nm,_,_) => nm,
      Self::Def(nm,_) => nm,
      Self::Anon(_) => ""
    }
  }
}

