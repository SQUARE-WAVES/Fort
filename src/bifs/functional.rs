use std::sync::Arc;
use super::{
  V,ExtType,
  F,
  Thread,
  Error,
  tpop
};

pub fn map<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let proc : F<E> = tpop(stk,"proc")?;
  let lst : Arc<[V<E>]> = tpop(stk,"list")?;
  
  th.start_list();

  for val in lst.iter() {
    th.push_val(val.clone());
    proc.run(th)?;
  };

  th.end_list().expect("PANIC, list didn't end for map fn");

  Ok(())
}

pub fn call<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let proc : F<E> = tpop(stk,"proc")?;
  proc.run(th)?;
  Ok(())
}

//the "if" function
pub fn cond<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk = th.stk();
  let else_proc : F<E> = tpop(stk,"else_proc")?;
  let true_proc : F<E> = tpop(stk,"true_proc")?;
  let val :bool = tpop(stk,"truth val")?;

  if val {
    true_proc.run(th)?;
  }
  else {
    else_proc.run(th)?;
  }

  Ok(())
}

pub fn while_loop<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let body : F<E> = tpop(th.stk(),"loop_body")?;
  let test : F<E> = tpop(th.stk(),"loop_test")?;
  
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
