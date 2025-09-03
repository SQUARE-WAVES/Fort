use crate::{
  V,
  F,
  Dict,
  traits::Fort,
  vm::{
    Thread,
    Error as VMErr,
  },
  tokens::{
    Token as T,
    St,
    Scanner,
    Error as TokErr
  }
};

pub struct Repl<S:Fort> {
  st:St,
  pos:usize,
  _g:std::marker::PhantomData<S>
}

impl<S:Fort> Repl<S> {
  pub fn new() -> Self {
    Self {
      st:Default::default(),
      pos:Default::default(),
      _g:Default::default()
    }
  }

  pub fn buff(&mut self,vm:&mut Thread<S> ,buff:&mut String) -> Result<(),Error> {
    let input = &buff[..];
    let mut lx = Scanner::resume(input,self.pos,self.st);

    loop {
      let tk = match lx.eat() {
        Ok(tk) => tk,
        Err(TokErr::Done) => {
          let(st,eaten,pos) = lx.done();
          self.st = st;
          self.pos = pos;
          buff.drain(0..eaten);
          return Ok(())
        },
        Err(e) => {
          self.st = St::Base;
          self.pos = 0;
          buff.clear();
          vm.drop_modes();
          return Err(Error::Scanner(e))
        }
      };

      match push_token(vm,tk) {
        Ok(()) => (),
        Err(e) => {
          self.st = St::Base;
          self.pos = 0;
          buff.clear();
          return Err(Error::Exec(e))
        }
      }
    }
  }
}

pub fn load_file<S,P>(p:P,d:&mut Dict<S>) -> Result<F<S>,Error> 
where 
  S:Fort, 
  P:AsRef<std::path::Path>
{
  let f = std::fs::read_to_string(p).map_err(Error::Fs)?;
  let mut vm = Thread::as_def(S::default_env(),d);
  let mut lx = Scanner::resume(&f[..],0,St::Base);

  let th = loop {
    let tk = match lx.eat() {
      Ok(tk) => tk,

      Err(TokErr::Done) => {
        break vm; 
      },

      Err(e) => {
        let (_,comp,pos) = lx.done();
        /*
        let end = comp+pos;
        let read = &f[..end];
        let ln = read.lines().count() + 1;
        let lp = read.rfind('\n').map(|p|end -p).unwrap_or(end);
        println!("error on line {ln} position {lp}");
        */
        return Err(Error::FileSrc(e,comp,comp+pos));
      }
    };

    match push_token(&mut vm,tk) {
      Ok(()) => (),
      Err(e) => {
        let (_,comp,pos) = lx.done();
        /*
        let end = comp+pos;
        let read = &f[..end];
        let ln = read.lines().count() + 1;
        let lp = read.rfind('\n').map(|p|end -p).unwrap_or(end);
        println!("error on line {ln} position {lp}");
        */

        return Err(Error::FileExec(e,comp,comp+pos));
      }
    };
  };

  let vf = th.into_function().map_err(Error::FileEnd)?;

  Ok(vf)
}

fn push_token<S:Fort>(vm:&mut Thread<S>,tk:T) -> Result<(),VMErr> {
  let res = match tk {
    T::OpenParen => {
      vm.start_def();
      Ok(())
    },
    
    T::CloseParen => vm.end_def(None),
    T::CloseDef(nm) => vm.end_def(Some(nm)),

    T::OpenBracket => {
      vm.start_list();
      Ok(())
    },
    T::CloseBracket => vm.end_list(),

    T::True => { 
      vm.push_val(V::B(true));
      Ok(())
    },
    T::False => {
      vm.push_val(V::B(false)); 
      Ok(())
    },
    T::I(i) =>{ 
      vm.push_val(V::I(i)); 
      Ok(())
    },
    T::Z(z) =>{ 
      vm.push_val(V::Z(z));
      Ok(())
    },
    T::Str(s) => {
      vm.push_val(V::Str(s.into()));
      Ok(())
    }

    T::Word(nm) => vm.word(nm),
    T::QWord(nm) => vm.quote(nm)
  };

  if res.is_err() {
    vm.drop_modes();
  }

  res
}

#[derive(Debug)]
pub enum Error {
  Fs(std::io::Error),
  FileSrc(TokErr,usize,usize),
  FileExec(VMErr,usize,usize),
  FileEnd(VMErr),
  Scanner(TokErr),
  Exec(VMErr)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::FileSrc(x,s,e) => write!(f,"scan error in file pos: {s}-{e} : {x}"),
      Self::FileExec(x,s,e) => write!(f,"error executing tokens in file pos {s}-{e} : {x}"),
      Self::FileEnd(x) => write!(f,"error executing tokens at end of file: {x}"),
      Self::Scanner(e) => write!(f,"error while scanning tokens {e}"),
      Self::Exec(e) => write!(f,"error in vm execution {e}"),
      Self::Fs(e)=> write!(f,"error reading file: {e}"),
    }
  }
}

impl std::error::Error for Error{}
