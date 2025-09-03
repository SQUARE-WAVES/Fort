use std::sync::Arc;

use crate::{
  V,ExtType,
  Thread,
  bifs::Error as BifError
};

pub type Bif<Ext:ExtType> = fn(&mut Thread<Ext>) -> Result<(),BifError>;

#[derive(Debug,Clone)]
pub enum F<Ext:ExtType> {
  Bif(&'static str,&'static str,Bif<Ext>),
  Def(Arc<str>,Arc<[V<Ext>]>),
  Anon(Arc<[V<Ext>]>)
}

impl<Ext:ExtType> std::fmt::Display for F<Ext> {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Bif(nm,_,_) => write!(f,"{nm}"),
      Self::Def(nm,_) => write!(f,"{nm}"),
      Self::Anon(_) => write!(f,"fn"),
    }
  }
}

impl<Ext:ExtType> PartialEq for F<Ext> {
  fn eq(&self,other:&F<Ext>) -> bool {
    match (self,other) {
      (Self::Bif(_,_,n), Self::Bif(_,_,m)) => std::ptr::fn_addr_eq(*n,*m),
      (Self::Def(_,a), Self::Def(_,b)) => Arc::ptr_eq(a,b),
      (Self::Anon(a), Self::Anon(b)) => Arc::ptr_eq(a,b),
      _ => false
    }
  }
}

impl<Ext:ExtType> F<Ext> {
  pub fn run(&self,th:&mut Thread<Ext>) -> Result<(),BifError> {
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

