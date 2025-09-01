use crate::{
  VM,
  vm::Error as VMErr,
  tokens::{
    St,
    Scanner,
    Error as TokErr
  }
};

#[derive(Default)]
pub struct Repl {
  st:St,
  pos:usize
}

impl Repl {
  pub fn buff(&mut self,vm:&mut VM,buff:&mut String) -> Result<(),Error> {
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

      match vm.push_token(tk) {
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
#[derive(Debug,Copy,Clone)]
pub enum Error {
  Scanner(TokErr),
  Exec(VMErr)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Scanner(e) => write!(f,"error while scanning tokens {e}"),
      Self::Exec(e) => write!(f,"error in vm execution {e}"),
    }
  }
}

impl std::error::Error for Error{}
