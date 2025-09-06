use std::{
  fmt::{
    Debug,
    Display,
    Formatter,
    Error as FmtErr
  },
  sync::Arc
};

use crate::{
  V,
  Thread,
  bifs::{
    Bif,
    Error as BifError
  },
  traits::Fort
};

pub enum F<S:Fort> {
  Bif(&'static str,&'static str,Bif<S>),
  Def(Arc<str>,Arc<[V<S>]>),
  Anon(Arc<[V<S>]>)
}

impl<S:Fort> Display for F<S> {
  fn fmt(&self,f:&mut Formatter) -> Result<(),FmtErr> {
    match self {
      Self::Bif(nm,_,_) => write!(f,"{nm}"),
      Self::Def(nm,_) => write!(f,"{nm}"),
      Self::Anon(_) => write!(f,"fn"),
    }
  }
}

impl<S:Fort> Debug for F<S> {
  fn fmt(&self,f:&mut Formatter) -> Result<(),FmtErr> {
    match self {
      Self::Bif(nm,_,_) => write!(f,"F::Bif({nm})"),
      Self::Def(nm,_) => write!(f,"F:Def({nm})"),
      Self::Anon(_) => write!(f,"F::Anon"),
    }
  }
}

impl<S:Fort> Clone for F<S> {
  fn clone(&self) -> Self {
    match self {
      Self::Bif(nm,doc,f) => Self::Bif(nm,doc,f.clone()),
      Self::Def(nm,vs) => Self::Def(nm.clone(),vs.clone()),
      Self::Anon(vs) => Self::Anon(vs.clone())
    }
  }
}

impl<S:Fort> PartialEq for F<S> {
  fn eq(&self,other:&F<S>) -> bool {
    match (self,other) {
      (Self::Bif(_,_,n), Self::Bif(_,_,m)) => n==m,
      (Self::Def(_,a), Self::Def(_,b)) => Arc::ptr_eq(a,b),
      (Self::Anon(a), Self::Anon(b)) => Arc::ptr_eq(a,b),
      _ => false
    }
  }
}

impl<S:Fort> F<S> {
  pub fn run(&self,th:&mut Thread<S>) -> Result<(),BifError> {
    match self {
      Self::Bif(_,_,f) => f.call(th),

      Self::Def(_,arc) => {
        for v in arc.iter() {
          match v {
            V::C(f) => {
              f.run(th)?;
            }
            x => {
              th.push(x.clone());
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
              th.push(x.clone())
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

