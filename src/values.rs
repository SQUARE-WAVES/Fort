use std::sync::Arc;

use crate::{
  Vstack,
  bifs::Error as BifError
};

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

#[derive(Debug,Clone,PartialEq)]
pub enum V {
  Z(f64),
  L(Arc<[V]>),
  I(i64),
  F(F),
  C(F),
  B(bool),
}

impl V {
  pub fn type_tag(&self) -> &'static str {
    match self {
      V::Z(_) => "float",
      V::L(_) => "list",
      V::I(_) => "int",
      V::F(_) => "function",
      V::C(_) => "function call",
      V::B(_) => "bool",
    }
  }
}

pub trait VType:TryFrom<V,Error=V> + Into<V> {
  fn type_tag() -> &'static str;
}

impl std::fmt::Display for V {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      
      Self::Z(n) => write!(f,"{n}"),

      Self::L(vs) => {
        let mut vi = vs.iter();
        let first = vi.next();
        if let Some(v) = first {
          write!(f,"[{v}")?;
          for v in vi {
            write!(f," {v}")?;
          }
        }
        else {
          write!(f,"[")?;
        }

        write!(f,"]")
      },

      Self::I(n) => write!(f,"{n}"),

      Self::F(_) => write!(f,"fn"),

      Self::C(_) => write!(f,"thnk"),
      Self::B(n) => write!(f,"{n}"),
    }
  }
}

macro_rules! vtraits {
  ($TYP:ty, $VTYP:ident,$tag:literal) => {
    impl From<$TYP> for V {
      fn from(t:$TYP) -> Self {
        V::$VTYP(t)
      }
    }

    impl TryFrom<V> for $TYP {
      type Error=V;
      fn try_from(v:V) -> Result<Self,V> {
        match v {
          V::$VTYP(n) => Ok(n),
          _ => Err(v)
        }
      }
    }

    impl VType for $TYP {
      fn type_tag() -> &'static str {
        $tag
      }
    }
  }
}

vtraits!{f64,Z,"float"}
vtraits!{Arc<[V]>,L,"list"}
vtraits!{i64,I,"int"}
vtraits!{F,F,"function"}
vtraits!{bool,B,"bool"}
