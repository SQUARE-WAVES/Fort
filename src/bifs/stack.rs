use super::{
  Vstack,
  Error,
  //err
};

pub fn dup(stk:&mut Vstack) -> Result<(),Error> {
  let v = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(v.clone());
  stk.push(v.clone());
  Ok(())
}

pub fn over(stk:&mut Vstack) -> Result<(),Error> {
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  let ac = a.clone();
  stk.push(a);
  stk.push(b);
  stk.push(ac);
  Ok(())
}

pub fn swap(stk:&mut Vstack) -> Result<(),Error> {
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(b);
  stk.push(a);
  Ok(())
}

pub fn rot(stk:&mut Vstack) -> Result<(),Error> {
  let c = stk.pop().map_err(|se|Error::Param("c",se))?;
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(b);
  stk.push(c);
  stk.push(a);
  Ok(())
}

pub fn drop(stk:&mut Vstack) -> Result<(),Error> {
  let _ = stk.pop();
  Ok(())
}

pub fn clear(stk:&mut Vstack) -> Result<(),Error> {
  stk.clear();
  Ok(())
}
