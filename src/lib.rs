//OK here is the virtual machine stuff, nothing to do with parsing or tokens or whaver
pub mod traits;
pub mod values;
pub mod text;
pub mod functions;
pub mod dictionary;
pub mod frame_stack;
pub mod vm;

//here is the built in stuff that helps you do things
pub mod bifs;
pub mod tokens;
pub mod parser;

pub use {
  traits::{
    Fort,
    TypeTag,
    TaggedType
  },
  values::V,
  text::{Txt,Sym},
  functions::F,
  dictionary::{Dict,Scope},
  frame_stack::FrameStack,
  vm::Thread
};
