use super::{
  V,
  Vstack,
  Thread,
  Error,
  p_err,
  ptyp_err,
  punder
};

//this exists to avoid duplicating the type checking stuff
//for every single math function
fn two_op<II,FF>(stk:&mut Vstack,is:II,fs:FF) -> Result<(),Error> 
where 
    II:FnOnce(i64,i64)->i64,
    FF:FnOnce(f64,f64)->f64
{
  match stk.peek(2) {
    [V::Z(a),V::Z(b)] => {
      let res = fs(*a,*b);
      stk.dropn(2);
      stk.push(res);
      Ok(())
    },
    [V::I(a),V::I(b)] => {
      let res = is(*a,*b);
      stk.dropn(2);
      stk.push(res);
      Ok(())
    },
    [V::Z(_),s] => Err(ptyp_err("b","float",s.type_tag())),
    [V::I(_),s] => Err(ptyp_err("b","int",s.type_tag())),
    [s,_] => Err(ptyp_err("a","float or int",s.type_tag())),
    [_] => Err(punder("a")),
    [] => Err(punder("b")),
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
  let a = stk.pop().map_err(p_err("a"))?;
  let b = stk.pop().map_err(p_err("b"))?;
  stk.push(a==b);

  Ok(())
}

pub fn neq(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let a = stk.pop().map_err(p_err("a"))?;
  let b = stk.pop().map_err(p_err("b"))?;
  stk.push(a != b);

  Ok(())
}

pub fn to_int(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let a = stk.pop().map_err(p_err("a"))?;
  let b = stk.pop().map_err(p_err("b"))?;
  stk.push(a != b);

  Ok(())
}

pub fn to_float(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let a = stk.tpop::<i64>().map_err(p_err("a"))?;
  let z :f64 = a as f64;
  stk.push(z);
  Ok(())
}
