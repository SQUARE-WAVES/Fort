use std::sync::Arc;
use super::{
  V,
  Fort,
  F,
  Thread,
  Error,
  tpop
};

pub fn map<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let proc : F<S> = tpop(stk,"proc")?;
  let lst : Arc<[V<S>]> = tpop(stk,"list")?;
  
  th.start_list();

  for val in lst.iter() {
    th.push_val(val.clone());
    proc.run(th)?;
  };

  th.end_list().expect("PANIC, list didn't end for map fn");

  Ok(())
}

pub fn call<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let proc : F<S> = tpop(stk,"proc")?;
  proc.run(th)?;
  Ok(())
}

//the "if" function
pub fn cond<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let stk = th.stk();
  let else_proc : F<S> = tpop(stk,"else_proc")?;
  let true_proc : F<S> = tpop(stk,"true_proc")?;
  let val :bool = tpop(stk,"truth val")?;

  if val {
    true_proc.run(th)?;
  }
  else {
    else_proc.run(th)?;
  }

  Ok(())
}

pub fn while_loop<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let body : F<S> = tpop(th.stk(),"loop_body")?;
  let test : F<S> = tpop(th.stk(),"loop_test")?;
  
  loop {
    let v = th.stk().popv().ok_or(Error::Internal("couldn't dup body output in while loop"))?;
    th.push_val(v.clone());
    th.push_val(v);

    test.run(th)?;
    let tr : bool = tpop(th.stk(),"loop test variable")?;
    if tr {
      body.run(th)?;
    }
    else {
      break
    };
  }

  Ok(())
}
