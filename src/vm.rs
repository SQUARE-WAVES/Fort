use std::sync::Arc;

use crate::{
  V,F,
  Vstack,
  Dict,
  tokens::Token as T
};

#[derive(Debug)]
pub enum Mode {
  List(Vstack),
  Def(Vstack),
}

impl Default for Mode {
  fn default() -> Self {
    Self::list()
  }
}

impl Mode {
  pub fn def() -> Self {
    Self::Def(Default::default())
  }

  pub fn list() -> Self {
    Self::List(Default::default())
  }

  pub fn vs(&mut self) -> &mut Vstack {
    match self {
      Self::List(stk) => stk,
      Self::Def(stk) => stk
    }
  }

  pub fn push_value(&mut self,val:V) {
    self.vs().push(val)
  }
}

impl std::fmt::Display for Mode {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::List(vs) => write!(f,"[{vs}"),
      Self::Def(vs) => write!(f,"({vs}")
    }
  }
}

#[derive(Default)]
pub struct VM {
  root:Mode,
  mode_stack:Vec<Mode>,
  dict:Dict
}

impl VM {
  pub fn drop_modes(&mut self) {
    self.mode_stack.clear()
  }

  //basic debug printing type stuff
  pub fn print(&mut self) {
    if self.root.vs().is_empty() && self.mode_stack.is_empty() {
      println!("-=EMPTY=-");
    }
    else {
      print!("-= {} ",self.root.vs());
      for md in self.mode_stack.iter() {
        print!("{md}");
      }
      println!("=-");
    }
  }

  fn mode(&mut self) -> &mut Mode {
    self.mode_stack.last_mut().unwrap_or(&mut self.root)
  }

  fn push_def(&mut self) { 
    self.mode_stack.push(Mode::def());
    self.dict.push_scope();
  }

  fn push_list(&mut self) { self.mode_stack.push(Mode::list()) }

  pub fn push_value(&mut self,val:V) {
    self.mode().push_value(val)
  }

  fn push_call(&mut self,call:F) -> Result<(),Error> {
    let md = self.mode();
    match md {
      Mode::Def(vs) => {
        vs.push(V::C(call));
        Ok(())
      },
      Mode::List(vs) => call.run(vs).map_err(|_|Error::Bad(1)),
    }
  }

  fn push_quote(&mut self,qw:&str) -> Result<(),Error> {
    let q = self.dict.get(qw).map_err(|_|Error::Bad(3))?;
    self.push_value(V::F(q.clone()));
    Ok(())
  }

  fn push_word(&mut self,nm:&str) -> Result<(),Error> {
    let q = self.dict.get(nm).map_err(|_|Error::Bad(3))?;
    self.push_call(q.clone())?;
    Ok(())
  }

  fn end_list(&mut self) -> Result<(),Error> {
    match self.mode_stack.pop() {
      Some(Mode::List(vs)) => {
        self.push_value(V::L(vs.into()));
        Ok(())
      },
      Some(Mode::Def(_)) => {
        self.drop_modes();
        Err(Error::Bad(10))
      },
      None => Err(Error::Bad(11))
    }
  }

  fn end_def<T1:Into<Option<Arc<str>>>>(&mut self,nm:T1) -> Result<(),Error> {
    let vs = match self.mode_stack.pop() {
      Some(Mode::Def(stk)) => Ok(stk),
      _ => Err(Error::Bad(20))
    }?;

    self.dict.pop_scope();

    let f = self.dict.define(vs.into(),nm);
    if matches!(f,F::Anon(_)) {
      self.push_value(V::F(f))
    }

    Ok(())
  }

  pub fn push_token(&mut self,tok:T) -> Result<(),Error> {
    let res = match tok {
      T::OpenParen => {
        self.push_def();
        Ok(())
      },
      
      T::CloseParen => self.end_def(None),
      T::CloseDef(nm) => self.end_def::<Arc<str>>(nm.into()),

      T::OpenBracket => {
        self.push_list();
        Ok(())
      },
      T::CloseBracket => self.end_list(),

      T::True => { 
        self.push_value(V::B(true));
        Ok(())
      },
      T::False => {
        self.push_value(V::B(false)); 
        Ok(())
      },
      T::I(i) =>{ 
        self.push_value(V::I(i)); 
        Ok(())
      },
      T::Z(z) =>{ 
        self.push_value(V::Z(z));
        Ok(())
      },
      T::Str(s) => {
        self.push_value(V::Str(s.into()));
        Ok(())
      }

      T::Word(nm) => self.push_word(nm),
      T::QWord(nm) => self.push_quote(nm),
      T::Print => {
        self.print();
        Ok(())
      }
    };

    if res.is_err() {
      self.drop_modes();
    }

    res
  }
}

#[derive(Clone,Copy,Debug)]
pub enum Error {
  Bad(usize)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Bad(u) => write!(f,"VM TEMPORARY ERR CODE {u}")
    }
  }
}

