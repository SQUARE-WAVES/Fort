use super::{
  Thread,
  Error,
  param
};

pub fn dup(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let v = param(stk,"a")?;
  stk.push(v.clone());
  stk.push(v.clone());
  Ok(())
}

pub fn over(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  let ac = a.clone();
  stk.push(a);
  stk.push(b);
  stk.push(ac);
  Ok(())
}

pub fn swap(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  stk.push(b);
  stk.push(a);
  Ok(())
}

pub fn rot(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let c = param(stk,"c")?;
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  stk.push(b);
  stk.push(c);
  stk.push(a);
  Ok(())
}

pub fn drop(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let _ = stk.popv();
  Ok(())
}

pub fn clear(th:&mut Thread) -> Result<(),Error> {
  th.stk().clear();
  Ok(())
}
