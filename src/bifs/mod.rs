mod stack;

pub enum BifError{
  Bad
}

use crate::{V,F,Vstack};

pub fn add(stk:&mut Vstack) -> Result<(),BifError> {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (b,a) {
    (V::I(n),V::I(d)) => stk.push(V::I(n + d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n + d)),
    _ => panic!("invalid multipication types, must be int + int or z + z")
  }

  Ok(())
}

pub fn sub(stk:&mut Vstack) -> Result<(),BifError> {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (a,b) {
    (V::I(n),V::I(d)) => stk.push(V::I(n - d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n - d)),
    _ => panic!("invalid subtraction types, must be int - int or z - z")
  }

  Ok(())
}

pub fn mul(stk:&mut Vstack) -> Result<(),BifError> {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (b,a) {
    (V::I(n),V::I(d)) => stk.push(V::I(n * d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n * d)),
    _ => panic!("invalid multipication types, must be int * int or z * z")
  }

  Ok(())
}

pub fn div(stk:&mut Vstack) -> Result<(),BifError> {
  let den = stk.pop().expect("stack underflow");
  let num = stk.pop().expect("stack underflow");
  match (num,den) {
    (V::I(n),V::I(d)) => stk.push(V::I(n/d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n/d)),
    _ => panic!("invalid divison types, must be int/int or z/z")
  }

  Ok(())
}

pub fn eq(stk:&mut Vstack) -> Result<(),BifError> {
  let a = stk.pop().expect("stack underflow");
  let b = stk.pop().expect("stack underflow");
  match (a,b) {
    (V::I(n),V::I(d)) => stk.push(V::B(n == d)),
    (V::Z(n),V::Z(d)) => stk.push(V::B(n == d)),
    (V::B(n),V::B(d)) => stk.push(V::B(n == d)),
    (V::L(n),V::L(d)) => stk.push(V::B(std::sync::Arc::ptr_eq(&n,&d))),
    (V::F(n),V::F(d)) => stk.push(V::B(n==d)),
    _ => panic!("invalid comparison types, both args must be the same type")
  }

  Ok(())
}

pub fn neq(stk:&mut Vstack) -> Result<(),BifError> {
  let a = stk.pop().expect("stack underflow");
  let b = stk.pop().expect("stack underflow");
  match (a,b) {
    (V::I(n),V::I(d)) => stk.push(V::B(n != d)),
    (V::Z(n),V::Z(d)) => stk.push(V::B(n != d)),
    (V::B(n),V::B(d)) => stk.push(V::B(n != d)),
    (V::L(n),V::L(d)) => stk.push(V::B(!std::sync::Arc::ptr_eq(&n,&d))),
    (V::F(n),V::F(d)) => stk.push(V::B(n != d)),
    _ => panic!("invalid comparison types, both args must be the same type")
  }

  Ok(())
}

pub fn map(stk:&mut Vstack) -> Result<(),BifError> {
  let proc = stk.popf().expect("couldn't get proc to map");
  let lst = stk.popl().expect("coultn't get list");
  let mut subs = Vstack::default();
  for val in lst.iter() {
    subs.push(val.clone());
    proc.run(&mut subs)?;
  };

  let newl = subs.lst(0);
  stk.push(newl);
  Ok(())
}

pub fn call(stk:&mut Vstack) -> Result<(),BifError> {
  let proc = stk.popf().expect("couldn't get proc to call");
  proc.run(stk)?;
  Ok(())
}

pub fn cond(stk:&mut Vstack) -> Result<(),BifError> {
  let else_proc = stk.popf().expect("couldn't get proc to call");
  let true_proc = stk.popf().expect("couldn't get proc to call");
  let val = stk.popb().expect("couldn't get boolean value");

  if val {
    true_proc.run(stk)?;
  }
  else {
    else_proc.run(stk)?;
  }

  Ok(())
}

pub fn while_loop(stk:&mut Vstack) -> Result<(),BifError> {
  let body = stk.popf().expect("couln't find a thing to do in the loop");
  let test = stk.popf().expect("couldn't find thing to test for the loop");
  
  loop {
    let v = stk.pop().expect("couldn't dup");
    stk.push(v.clone());
    stk.push(v);
    test.run(stk);
    match stk.popb() {
      Some(true) => body.run(stk),
      _ => break
    };
  }

  Ok(())
}

pub fn print_stack(stk:&mut Vstack) -> Result<(),BifError> {
  stk.print();
  println!();
  Ok(())
}

pub fn root_dict() -> std::collections::HashMap<String,F> {
  std::collections::HashMap::from([
    ("dup".into(),F::Bif(stack::dup)),
    ("+".into(),F::Bif(add)),
    ("-".into(),F::Bif(sub)),
    ("*".into(),F::Bif(mul)),
    ("/".into(),F::Bif(div)),
    ("map".into(),F::Bif(map)),
    ("call".into(),F::Bif(call)),
    (".".into(),F::Bif(print_stack)),
    ("==".into(),F::Bif(eq)),
    ("!=".into(),F::Bif(neq)),
    ("if".into(),F::Bif(cond)),
    ("while".into(),F::Bif(while_loop))
  ])
}
