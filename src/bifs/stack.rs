use super::{
  Thread,
  ExtType,
  Error,
  param
};

pub fn dup<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let v = param(stk,"a")?;
  stk.push(v.clone());
  stk.push(v.clone());
  Ok(())
}

pub fn over<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  let ac = a.clone();
  stk.push(a);
  stk.push(b);
  stk.push(ac);
  Ok(())
}

pub fn swap<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  stk.push(b);
  stk.push(a);
  Ok(())
}

pub fn rot<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let c = param(stk,"c")?;
  let b = param(stk,"b")?;
  let a = param(stk,"a")?;
  stk.push(b);
  stk.push(c);
  stk.push(a);
  Ok(())
}

pub fn drop<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let _ = stk.popv();
  Ok(())
}

pub fn clear<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  th.stk().clear();
  Ok(())
}
