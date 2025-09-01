use std::sync::Arc;
use super::{
  V,
  F,
  Vstack,
  Error,
  p_err
};

pub fn map(stk:&mut Vstack) -> Result<(),Error> {
  let proc = stk.tpop::<F>().map_err(p_err("proc"))?;
  let lst = stk.tpop::<Arc<[V]>>().map_err(p_err("list"))?;
  
  let mut subs = Vstack::default();

  for val in lst.iter() {
    subs.push(val.clone());
    proc.run(&mut subs)?;
  };

  let newl = subs.lst(0);
  stk.push(newl);
  Ok(())
}

pub fn call(stk:&mut Vstack) -> Result<(),Error> {
  let proc = stk.tpop::<F>().map_err(p_err("process"))?;
  proc.run(stk)?;
  Ok(())
}


//the "if" function
pub fn cond(stk:&mut Vstack) -> Result<(),Error> {
  let else_proc = stk.tpop::<F>().map_err(p_err("else_proc"))?;
  let true_proc = stk.tpop::<F>().map_err(p_err("true_proc"))?;
  let val = stk.tpop::<bool>().map_err(p_err("truth val"))?;

  if val {
    true_proc.run(stk)?;
  }
  else {
    else_proc.run(stk)?;
  }

  Ok(())
}

pub fn while_loop(stk:&mut Vstack) -> Result<(),Error> {
  let body = stk.tpop::<F>().map_err(p_err("loop body"))?;
  let test = stk.tpop::<F>().map_err(p_err("loop test"))?;
  
  loop {
    let v = stk.pop().expect("couldn't dup");
    stk.push(v.clone());
    stk.push(v);
    test.run(stk)?;
    let tr = stk.tpop::<bool>().map_err(p_err("loop test variable"))?;
    if tr {
      body.run(stk)?;
    }
    else {
      break
    };
  }

  Ok(())
}
