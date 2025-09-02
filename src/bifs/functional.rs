use std::sync::Arc;
use super::{
  V,
  F,
  Thread,
  Error,
  p_err
};

pub fn map(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let proc = stk.tpop::<F>().map_err(p_err("proc"))?;
  let lst = stk.tpop::<Arc<[V]>>().map_err(p_err("list"))?;
  
  th.start_list();

  for val in lst.iter() {
    th.push_val(val.clone());
    proc.run(th)?;
  };

  th.end_list().expect("PANIC, list didn't end for map fn");

  Ok(())
}

pub fn call(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let proc = stk.tpop::<F>().map_err(p_err("process"))?;
  proc.run(th)?;
  Ok(())
}


//the "if" function
pub fn cond(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let else_proc = stk.tpop::<F>().map_err(p_err("else_proc"))?;
  let true_proc = stk.tpop::<F>().map_err(p_err("true_proc"))?;
  let val = stk.tpop::<bool>().map_err(p_err("truth val"))?;

  if val {
    true_proc.run(th)?;
  }
  else {
    else_proc.run(th)?;
  }

  Ok(())
}

pub fn while_loop(th:&mut Thread) -> Result<(),Error> {
  let body = th.stk().tpop::<F>().map_err(p_err("loop body"))?;
  let test = th.stk().tpop::<F>().map_err(p_err("loop test"))?;
  
  loop {
    {
      let stk = th.stk();
      let v = stk.pop().expect("couldn't dup");
      th.push_val(v.clone());
      th.push_val(v);
    }

    test.run(th)?;
    let tr = th.stk().tpop::<bool>().map_err(p_err("loop test variable"))?;
    if tr {
      body.run(th)?;
    }
    else {
      break
    };
  }

  Ok(())
}
