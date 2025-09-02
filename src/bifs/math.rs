use super::{
  V,
  Vstack,
  Thread,
  Error,
  param,
  tpop
};

//this exists to avoid duplicating the type checking stuff
//for every single math function
fn two_op<II,FF>(stk:&mut Vstack<V>,is:II,fs:FF) -> Result<(),Error> 
where 
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
    [V::Z(_),s] => Err(Error::PType("b","float",s.type_tag())),
    [V::I(_),s] => Err(Error::PType("b","int",s.type_tag())),
    [s,_] => Err(Error::PType("a","float or int",s.type_tag())),
    [_] => Err(Error::Underflow("a")),
    [] => Err(Error::Underflow("b")),
    _ => panic!("peek returned impossible length")
  }
}

pub fn add(th:&mut Thread) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1+i2,|f1,f2|f1+f2)
}

pub fn sub(th:&mut Thread) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1-i2,|f1,f2|f1-f2)
}

pub fn mul(th:&mut Thread) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1*i2,|f1,f2|f1*f2)
}

pub fn div(th:&mut Thread) -> Result<(),Error> {
  two_op(th.stk(),|i1,i2|i1/i2,|f1,f2|f1/f2)
}

//for the booleans
pub fn eq(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let a = param(stk,"a")?;
  let b = param(stk,"b")?;
  stk.push(a==b);

  Ok(())
}

pub fn neq(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let a = param(stk,"a")?;
  let b = param(stk,"b")?;
  stk.push(a != b);

  Ok(())
}

//TODO::check for better casting, I think these can panic
pub fn to_int(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let z = tpop::<f64>(stk,"z")?;
  let i : i64 = z as i64;
  stk.push(i);
  Ok(())
}

pub fn to_float(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let i = tpop::<i64>(stk,"i")?;
  let z :f64 = i as f64;
  stk.push(z);
  Ok(())
}
