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
  let proc : F<S> = tpop(th,"proc")?;
  let lst : Arc<[V<S>]> = tpop(th,"list")?;
  
  th.start_list();

  for val in lst.iter() {
    th.push(val.clone());
    proc.run(th)?;
  };

  th.end_list().expect("PANIC, list didn't end for map fn");

  Ok(())
}

pub fn call<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let proc : F<S> = tpop(th,"proc")?;
  proc.run(th)?;
  Ok(())
}

//the "if" function
pub fn cond<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let else_proc : F<S> = tpop(th,"else_proc")?;
  let true_proc : F<S> = tpop(th,"true_proc")?;
  let val :bool = tpop(th,"truth val")?;

  if val {
    true_proc.run(th)?;
  }
  else {
    else_proc.run(th)?;
  }

  Ok(())
}

pub fn while_loop<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let body : F<S> = tpop(th,"loop_body")?;
  let test : F<S> = tpop(th,"loop_test")?;
  
  loop {
    let v = th.popv().ok_or(Error::Internal("couldn't dup body output in while loop"))?;
    th.push(v.clone());
    th.push(v);

    test.run(th)?;
    let tr : bool = tpop(th,"loop test variable")?;
    if tr {
      body.run(th)?;
    }
    else {
      break
    };
  }

  Ok(())
}
