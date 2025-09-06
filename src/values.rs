use std::fmt::{
  Debug,Display,
  Formatter,
  Error as FmtErr
};

use std::sync::Arc;

use crate::{
  Txt,Sym,F,
  traits::{
    Fort,
    TaggedType,
    TypeTag
  }
};

//these are the basic types that can be extended
#[derive(Debug)]
pub enum V<S:Fort> {
  Z(f64),
  L(Arc<[Self]>),
  I(i64),
  F(F<S>),
  C(F<S>),
  B(bool),
  Str(Txt),
  Sym(Sym),
  Ext(S::Extension)
}

impl<S:Fort> TypeTag for V<S> {
  fn tag(&self) -> &'static str {
    match self {
      V::Z(_) => "float",
      V::L(_) => "list",
      V::I(_) => "int",
      V::F(_) => "function",
      V::C(_) => "function call",
      V::B(_) => "bool",
      V::Str(_) => "string",
      V::Sym(_) => "symbol",
      V::Ext(n) => n.tag()
    }
  }
}

impl<S:Fort> Display for V<S> {
  fn fmt(&self,f:&mut Formatter) -> Result<(),FmtErr> {
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
      Self::Sym(n) => write!(f,"Sym< {n} >"),
      Self::Ext(n) => write!(f,"{n}")
    }
  }
}

impl<S:Fort> PartialEq for V<S> {
  fn eq(&self,o:&Self) -> bool {
    match (self,o) {
      (V::Z(v),V::Z(v2)) => v == v2, 
      (V::L(v),V::L(v2)) => v == v2,
      (V::I(v),V::I(v2)) => v == v2,
      (V::F(v),V::F(v2)) => v == v2,
      (V::C(v),V::C(v2)) => v == v2,
      (V::B(v),V::B(v2)) => v == v2,
      (V::Str(v),V::Str(v2)) => v == v2,
      (V::Sym(v),V::Sym(v2)) => v == v2,
      (V::Ext(v),V::Ext(v2)) => v == v2,
      _ => false
    }
  }
}

impl<S:Fort> Clone for V<S> {
  fn clone(&self) -> Self {
    match self {
      V::Z(v) => V::Z(*v),
      V::L(v) => V::L(v.clone()),
      V::I(v) => V::I(*v),
      V::F(v) => V::F(v.clone()),
      V::C(v) => V::C(v.clone()),
      V::B(v) => V::B(*v),
      V::Str(v) => V::Str(v.clone()),
      V::Sym(v) => V::Sym(v.clone()),
      V::Ext(v) => V::Ext(v.clone()),
    }
  }
}

macro_rules! vtraits {
  ($TYP:ty, $VTYP:ident,$tag:literal) => {
    impl<S:Fort> From<$TYP> for V<S> {
      fn from(t:$TYP) -> Self {
        V::$VTYP(t)
      }
    }

    impl<S:Fort> TryFrom<V<S>> for $TYP {
      type Error=V<S>;
      fn try_from(v:V<S>) -> Result<Self,V<S>> {
        match v {
          V::$VTYP(n) => Ok(n),
          _ => Err(v)
        }
      }
    }

    impl TaggedType for $TYP {
      fn type_tag() -> &'static str {
        $tag
      }
    }
  }
}

vtraits!{f64,Z,"float"}
vtraits!{i64,I,"int"}
vtraits!{bool,B,"bool"}
vtraits!{Txt,Str,"string"}
vtraits!{Sym,Sym,"symbol"}

//gotta do it manually for F and list for now
impl<S:Fort> TaggedType for Arc<[V<S>]> {
  fn type_tag() -> &'static str { "list" }
}

impl<S:Fort> TryFrom<V<S>> for Arc<[V<S>]> {
  type Error=V<S>;
  fn try_from(v:V<S>) -> Result<Arc<[V<S>]>,V<S>> {
    match v {
      V::L(n) => Ok(n),
      _ => Err(v)
    }
  }
}

impl<S:Fort> From<Arc<[V<S>]>> for V<S> {
  fn from(lst:Arc<[V<S>]>) -> V<S> { V::L(lst) }
}

impl<S:Fort> TaggedType for F<S> {
  fn type_tag() -> &'static str { "function" }
}

impl<S:Fort> TryFrom<V<S>> for F<S>{
  type Error=V<S>;
  fn try_from(v:V<S>) -> Result<F<S>,V<S>> {
    match v {
      V::F(n) => Ok(n),
      _ => Err(v)
    }
  }
}

impl<S:Fort> From<F<S>> for V<S> {
  fn from(f:F<S>) -> V<S> { V::F(f) }
}
