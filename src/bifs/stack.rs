use super::{
  Thread,
  Error
};

pub fn dup(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let v = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(v.clone());
  stk.push(v.clone());
  Ok(())
}

pub fn over(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  let ac = a.clone();
  stk.push(a);
  stk.push(b);
  stk.push(ac);
  Ok(())
}

pub fn swap(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(b);
  stk.push(a);
  Ok(())
}

pub fn rot(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let c = stk.pop().map_err(|se|Error::Param("c",se))?;
  let b = stk.pop().map_err(|se|Error::Param("b",se))?;
  let a = stk.pop().map_err(|se|Error::Param("a",se))?;
  stk.push(b);
  stk.push(c);
  stk.push(a);
  Ok(())
}

pub fn drop(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let _ = stk.pop();
  Ok(())
}

pub fn clear(th:&mut Thread) -> Result<(),Error> {
  th.stk().clear();
  Ok(())
}
