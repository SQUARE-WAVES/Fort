use crate::{V,Vstack};

pub fn add(stk:&mut Vstack) {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (b,a) {
    (V::I(n),V::I(d)) => stk.push(V::I(n + d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n + d)),
    _ => panic!("invalid multipication types, must be int + int or z + z")
  }
}

pub fn sub(stk:&mut Vstack) {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (b,a) {
    (V::I(n),V::I(d)) => stk.push(V::I(n - d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n - d)),
    _ => panic!("invalid subtraction types, must be int - int or z - z")
  }
}

pub fn mul(stk:&mut Vstack) {
  let b = stk.pop().expect("stack underflow");
  let a = stk.pop().expect("stack underflow");
  match (b,a) {
    (V::I(n),V::I(d)) => stk.push(V::I(n * d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n * d)),
    _ => panic!("invalid multipication types, must be int * int or z * z")
  }
}

pub fn div(stk:&mut Vstack) {
  let den = stk.pop().expect("stack underflow");
  let num = stk.pop().expect("stack underflow");
  match (num,den) {
    (V::I(n),V::I(d)) => stk.push(V::I(n/d)),
    (V::Z(n),V::Z(d)) => stk.push(V::Z(n/d)),
    _ => panic!("invalid divison types, must be int/int or z/z")
  }
}

pub fn map(stk:&mut Vstack) {
  let proc = stk.popf().expect("couldn't get proc to map");
  let lst = stk.popl().expect("coultn't get list");
  let mut subs = Vstack::default();
  for val in lst.iter() {
    subs.push(val.clone());
    proc.run(&mut subs);
  };

  let newl = subs.lst(0);
  stk.push(newl);
}



