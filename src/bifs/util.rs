use super::{
  Vstack,
  Error,
  p_err,
  F,
  V
};


fn print_vs(vs:&[V]) {
  if vs.is_empty() {
    println!("()");
  }
  else {
    let mut vi = vs.iter();
    let v1 = vi.next().unwrap();
    print!("({v1}");
    for v in vi { print!(" {v}")};
    println!(")");
  }
}

pub fn doc(stk:&mut Vstack) -> Result<(),Error> {
  let f = stk.tpop::<F>().map_err(p_err("proc"))?;
  match &f {
    F::Bif(nm,d,_) => {
      println!("[[ {nm} ]]");
      println!("{d}");
    }
    F::Def(nm,vs) => {
      println!("[[ {nm} ]]");
      print_vs(vs)
    }
    F::Anon(vs)=> {
      println!("[[anonymous fn]]");
      print_vs(vs)
    }
  }
  println!("-------------------");

  stk.push(f);
  Ok(())
}
