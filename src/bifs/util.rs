use super::{
  Thread,
  Fort,
  Error,
  F,
  V,
  Txt,
  tpop
};

fn print_vs<S:Fort>(vs:&[V<S>]) {
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

pub fn doc<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let f : F<S> = tpop(th,"proc")?;
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

  th.push(f);
  Ok(())
}

pub fn print<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  th.print();
  Ok(())
}

pub fn do_file<S:Fort>(th:&mut Thread<S>) -> Result<(),Error> {
  let path : Txt = tpop(th,"path")?;
  let path : &str = &path; //gotta do this for the as_ref trait to kick in
  match crate::parser::load_file(path,th) {
    Ok(_) => Ok(()),
    Err(e) => {
      println!("--file load error--");
      println!("{e:?}");
      Err(Error::Internal("file load error"))
    }
  }
}
