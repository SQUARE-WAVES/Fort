use super::{
  Thread,ExtType,
  Error,
  F,
  V,
  Txt,
  tpop
};

fn print_vs<E:ExtType>(vs:&[V<E>]) {
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

pub fn doc<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let stk=th.stk();
  let f : F<E> = tpop(stk,"proc")?;
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

pub fn print<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  th.print();
  Ok(())
}

pub fn do_file<E:ExtType>(th:&mut Thread<E>) -> Result<(),Error> {
  let path : Txt = tpop(th.stk(),"path")?;
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
