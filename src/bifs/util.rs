use super::{
  Thread,
  Error,
  F,
  V,
  tpop
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

pub fn doc(th:&mut Thread) -> Result<(),Error> {
  let stk=th.stk();
  let f = tpop::<F>(stk,"proc")?;
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

pub fn print(th:&mut Thread) -> Result<(),Error> {
  th.print();
  Ok(())
}

pub fn do_file(th:&mut Thread) -> Result<(),Error> {
  use std::sync::Arc;
  let path = tpop::<Arc<str>>(th.stk(),"path")?;
  let path : &str = &path; //gotta do this for the as_ref trait to kick in
  let d = th.dict();
  match crate::parser::load_file(path,d) {
    Ok(f) => f.run(th),
    Err(e) => {
      println!("--file load error--");
      println!("{e:?}");
      Err(Error::Internal("file load error"))
    }
  }
}
