use super::{
  V,
  Vstack,
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

pub fn add(stk:&mut Vstack) -> Result<(),Error> {
  two_op(stk,|i1,i2|i1+i2,|f1,f2|f1+f2)
}

pub fn sub(stk:&mut Vstack) -> Result<(),Error> {
  two_op(stk,|i1,i2|i1-i2,|f1,f2|f1-f2)
}

pub fn mul(stk:&mut Vstack) -> Result<(),Error> {
  two_op(stk,|i1,i2|i1*i2,|f1,f2|f1*f2)
}

pub fn div(stk:&mut Vstack) -> Result<(),Error> {
  two_op(stk,|i1,i2|i1/i2,|f1,f2|f1/f2)
}

//for the booleans
pub fn eq(stk:&mut Vstack) -> Result<(),Error> {
  let a = stk.pop().map_err(p_err("a"))?;
  let b = stk.pop().map_err(p_err("b"))?;
  stk.push(a==b);

  Ok(())
}

pub fn neq(stk:&mut Vstack) -> Result<(),Error> {
  let a = stk.pop().map_err(p_err("a"))?;
  let b = stk.pop().map_err(p_err("b"))?;
  stk.push(a != b);

  Ok(())
}
