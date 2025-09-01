use std::sync::Arc;
use crate::functions::F;

#[derive(Debug,Clone,PartialEq)]
pub enum V {
  Z(f64),
  L(Arc<[V]>),
  I(i64),
  F(F),
  C(F),
  B(bool),
  Str(Arc<str>)
  //Sym(Arc<str>),
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
      V::Str(_) => "string"
      //V::Sym(_) => "symbol"
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

      Self::F(func) => write!(f,"{func}"),
      Self::C(func) => write!(f,"{func}"),

      Self::B(n) => write!(f,"{n}"),
      Self::Str(n) => write!(f,r#""{n}""#),
      //Self::Sym(n) => write!(f,"Sym[[ {n} ]]"),
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
