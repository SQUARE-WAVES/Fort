use super::{
  V,
  Fort,
  Vstack,
  Thread,
  Error,
  param,
  tpop,
  TypeTag
};

//this exists to avoid duplicating the type checking stuff
//for every single math function
fn two_op<S,II,FF>(stk:&mut Vstack<V<S>>,is:II,fs:FF) -> Result<(),Error> 
where 
    S:Fort,
    II:FnOnce(i64,i64)->i64,
    FF:FnOnce(f64,f64)->f64
{
  match stk.peek(2) {
    [V::Z(a),V::Z(b)] => {
      let res = fs(*a,*b);
      let _ = stk.drain(2);
      stk.push(res);
      Ok(())
    },
    [V::I(a),V::I(b)] => {
      let res = is(*a,*b);
      let _ = stk.drain(2);
      stk.push(res);
      Ok(())
    },
    [V::Z(_),s] => Err(Error::PType("b","float",s.tag())),
    [V::I(_),s] => Err(Error::PType("b","int",s.tag())),
    [s,_] => Err(Error::PType("a","float or int",s.tag())),
    [_] => Err(Error::Underflow("a")),
    [] => Err(Error::Underflow("b")),
    _ => panic!("peek returned impossible length")
  }
}

pub fn add<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1+i2,|f1,f2|f1+f2)
}

pub fn sub<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1-i2,|f1,f2|f1-f2)
}

pub fn mul<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1*i2,|f1,f2|f1*f2)
}

pub fn div<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1/i2,|f1,f2|f1/f2)
}

//for the booleans
pub fn eq<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let a = param(stk,"a")?;
  let b = param(stk,"b")?;
  stk.push(a==b);

  Ok(())
}

pub fn neq<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let a = param(stk,"a")?;
  let b = param(stk,"b")?;
  stk.push(a != b);

  Ok(())
}

//TODO::check for better casting, I think these can panic
pub fn to_int<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let z :f64 = tpop(stk,"z")?;
  let i : i64 = z as i64;
  stk.push(i);
  Ok(())
}

pub fn to_float<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let i : i64= tpop(stk,"i")?;
  let z :f64 = i as f64;
  stk.push(z);
  Ok(())
}
