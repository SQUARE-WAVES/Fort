use crate::{V,F,Vstack};
use super::BifError;

pub fn dup(stk:&mut Vstack) -> Result<(),BifError> {
  let v = stk.pop().expect("stack underflow");
  stk.push(v.clone());
  stk.push(v.clone());
  Ok(())
}
