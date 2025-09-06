use std::fmt::{Debug,Display};

//these two are to make sure you can tell what type your error was
//when you get a wrong type off the stack

//this one is for types, like i64 would return "int" or something
pub trait TaggedType {
  fn type_tag() -> &'static str;
}

//this one is for values, and especially enums so like Enum::I(i64) can say "int"
pub trait TypeTag {
  fn tag(&self) -> &'static str;
}

//This is a type that value extensioins will need to be able to act like 
//values for a forth system
pub trait ExtensionType:TypeTag+Debug+Display+Clone+PartialEq {}

impl<T:TypeTag+Debug+Display+Clone+PartialEq> ExtensionType for T {}

//just a few to help things out
impl TypeTag for () {
  fn tag(&self) -> &'static str {
    "unit"
  }
}

//this is to make using the vstack nicer, you want your extension types to implement this
//though they don't have to
pub trait StackType<V>:TaggedType + TryFrom<V,Error=V> + Into<V> {}

impl<V,T:TaggedType + TryFrom<V,Error=V> + Into<V>> StackType<V> for T {}
