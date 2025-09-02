use std::fmt::{
  Debug,Display,
  Formatter,
  Error as FmtErr
};

use std::sync::Arc;
use crate::functions::F;

//-----------------------------------------------------------------------------
//OH BOY A BUNCH OF TRAITS

//these 2 are here to make sure when you get a type error it can tell you what
//was going on
pub trait TaggedType {
  fn type_tag() -> &'static str;
}

pub trait TypeTag {
  fn tag(&self) -> &'static str;
}
//This is a kind of type that can be added externally to the system
pub trait ExtType:TypeTag+Debug+Display+Clone+PartialEq {}

//this is a type that can be popped off the stack safely, you want your
//external type to implent both of these
pub trait VType<Ext:ExtType>:TaggedType + TryFrom<V<Ext>,Error=V<Ext>> + Into<V<Ext>> {}

//these are the basic types that can be extended
#[derive(Debug,Clone,PartialEq)]
pub enum V<Ext:ExtType> {
  Z(f64),
  L(Arc<[Self]>),
  I(i64),
  F(F<Ext>),
  C(F<Ext>),
  B(bool),
  Str(Arc<str>),
  Ext(Ext)
  //Sym(Arc<str>),
}

impl<Ext:ExtType> TypeTag for V<Ext> {
  fn tag(&self) -> &'static str {
    match self {
      V::Z(_) => "float",
      V::L(_) => "list",
      V::I(_) => "int",
      V::F(_) => "function",
      V::C(_) => "function call",
      V::B(_) => "bool",
      V::Str(_) => "string",
      V::Ext(n) => n.tag()
      //V::Sym(_) => "symbol"
    }
  }
}

impl<Ext:ExtType> Display for V<Ext> {
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
      Self::Ext(n) => write!(f,"{n}")
      //Self::Sym(n) => write!(f,"Sym[[ {n} ]]"),
    }
  }
}

macro_rules! vtraits {
  ($TYP:ty, $VTYP:ident,$tag:literal) => {
    impl<Ext:ExtType> From<$TYP> for V<Ext> {
      fn from(t:$TYP) -> Self {
        V::$VTYP(t)
      }
    }

    impl<Ext:ExtType> TryFrom<V<Ext>> for $TYP {
      type Error=V<Ext>;
      fn try_from(v:V<Ext>) -> Result<Self,V<Ext>> {
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

    impl<Ext:ExtType> VType<Ext> for $TYP {}
  }
}

vtraits!{f64,Z,"float"}
vtraits!{i64,I,"int"}
vtraits!{bool,B,"bool"}
vtraits!{Arc<str>,Str,"string"}


//gotta do it manually for F and list for now
impl<Ext:ExtType> TaggedType for Arc<[V<Ext>]> {
  fn type_tag() -> &'static str { "list" }
}

impl<Ext:ExtType> TryFrom<V<Ext>> for Arc<[V<Ext>]> {
  type Error=V<Ext>;
  fn try_from(v:V<Ext>) -> Result<Arc<[V<Ext>]>,V<Ext>> {
    match v {
      V::L(n) => Ok(n),
      _ => Err(v)
    }
  }
}

impl<Ext:ExtType> From<Arc<[V<Ext>]>> for V<Ext> {
  fn from(lst:Arc<[V<Ext>]>) -> V<Ext> { V::L(lst) }
}

impl<Ext:ExtType> VType<Ext> for Arc<[V<Ext>]> {}

impl<Ext:ExtType> TaggedType for F<Ext> {
  fn type_tag() -> &'static str { "function" }
}

impl<Ext:ExtType> TryFrom<V<Ext>> for F<Ext>{
  type Error=V<Ext>;
  fn try_from(v:V<Ext>) -> Result<F<Ext>,V<Ext>> {
    match v {
      V::F(n) => Ok(n),
      _ => Err(v)
    }
  }
}

impl<Ext:ExtType> From<F<Ext>> for V<Ext> {
  fn from(f:F<Ext>) -> V<Ext> { V::F(f) }
}

impl<Ext:ExtType> VType<Ext> for F<Ext> {}
