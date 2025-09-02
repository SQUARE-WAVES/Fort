use std::sync::Arc;
use super::{
  V,
  F,
  Thread,
  Error,
  tpop
};

pub fn map(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let proc = tpop::<F>(stk,"proc")?;
  let lst = tpop::<Arc<[V]>>(stk,"list")?;
  
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
  let proc = tpop::<F>(stk,"process")?;
  proc.run(th)?;
  Ok(())
}

//the "if" function
pub fn cond(th:&mut Thread) -> Result<(),Error> {
  let stk = th.stk();
  let else_proc = tpop::<F>(stk,"else_proc")?;
  let true_proc = tpop::<F>(stk,"true_proc")?;
  let val = tpop::<bool>(stk,"truth val")?;

  if val {
    true_proc.run(th)?;
  }
  else {
    else_proc.run(th)?;
  }

  Ok(())
}

pub fn while_loop(th:&mut Thread) -> Result<(),Error> {
  let body = tpop::<F>(th.stk(),"loop body")?;
  let test = tpop::<F>(th.stk(),"loop test")?;
  
  loop {
    let v = th.stk().popv().ok_or(Error::Internal("couldn't dup body output in while loop"))?;
    th.push_val(v.clone());
    th.push_val(v);

    test.run(th)?;
    let tr = tpop::<bool>(th.stk(),"loop test variable")?;
    if tr {
      body.run(th)?;
    }
    else {
      break
    };
  }

  Ok(())
}
