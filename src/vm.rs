use crate::{
  V,
  Vstack,
  Dict,
  tokens,
  tokens::Token,
  bifs
};

#[derive(Default)]
pub struct VM {
  vstk:Vstack,
  dict:Dict,

  //stuff for input 
  pstk:Vec<ParseSt>,
  st:tokens::St,
  pos:usize
}

impl VM {
  pub fn repl_buff(&mut self,buff:&mut String) -> Result<(),Error> {
    let input = &buff[..];
    let mut lx = tokens::Scanner::resume(input,self.pos,self.st);
    loop {
      let tk = match lx.eat() {
        Ok(tk) => tk,
        Err(tokens::Error::Done) => {
          let(st,eaten,pos) = lx.done();
          self.st = st;
          self.pos = pos;
          buff.drain(0..eaten);
          return Ok(())
        },
        Err(e) => {
          self.st = tokens::St::Base;
          self.pos = 0;
          buff.clear();
          return Err(Error::Scanner(e))
        }
      };

      match self.eat_token(tk) {
        Ok(()) => (),
        Err(e) => {
          self.st = tokens::St::Base;
          self.pos = 0;
          buff.clear();
          return Err(e)
        }
      }
    }
    /*
    let mut tks : Vec<Token> = vec![];
    let mut cb = |tk| {
      tks.push(tk);
    };

    for c in src.chars() {
      self.tkzer.eat(c,&mut cb).ok_or(Error::BadTok);
    }

    for tk in tks {
      self.eat_token(tk)?;
    }

    Ok(())
    */
  }

  fn eat_token(&mut self,t:Token) -> Result<(),Error> {
    match self.pstk.last() {
      Some(ParseSt::List(n)) => self.list_token(t,*n)?,
      Some(ParseSt::Def(n)) => self.def_token(t,*n)?,
      None => self.base_token(t)?,
    }

    Ok(())
  }

  fn base_token(&mut self,t:Token) -> Result<(),Error> {
    use Token as T;
  
    match t {
      T::OpenParen => {
        self.dict.push_scope();
        self.pstk.push(ParseSt::Def(self.vstk.len()))
      }
      T::OpenBracket => self.pstk.push(ParseSt::List(self.vstk.len())),
      T::CloseParen | T::CloseDef(_)| T::CloseBracket =>{ 
        Err(Error::CloseWithoutOpen)?
      },
      T::Z(v) => self.vstk.push(V::Z(v)),
      T::I(v) => self.vstk.push(V::I(v)),
      T::True => self.vstk.push(true),
      T::False => self.vstk.push(false),
      T::Word(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        f.run(&mut self.vstk).map_err(Error::Bif)?;
      },
      T::QWord(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()))
      },
      T::Str(s) => println!("str {s}")
    };

    Ok(())
  }

  fn list_token(&mut self,t:Token,pos:usize) -> Result<(),Error> {
    use Token as T;

    match t {
      T::OpenParen => {
        self.dict.push_scope();
        self.pstk.push(ParseSt::Def(self.vstk.len()))
      },
      
      T::OpenBracket => self.pstk.push(ParseSt::List(self.vstk.len())),
      
      T::CloseBracket => {
        let v = self.vstk.lst(pos);
        self.vstk.push(v);
        self.pstk.pop();
      }
      
      T::CloseParen | T::CloseDef(_) => { 
        Err(Error::CloseWithoutOpen)?
      },

      T::Z(v) => self.vstk.push(v),
      T::I(v) => self.vstk.push(v),
      T::True => self.vstk.push(true),
      T::False => self.vstk.push(false),

      T::Word(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        f.run(&mut self.vstk).map_err(Error::Bif)?;
      },

      T::QWord(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()))
      },

      T::Str(s) => println!("str {s}")
    };

    Ok(())
  }

  fn def_token(&mut self,t:Token,pos:usize) -> Result<(),Error> {
    use Token as T;
  
    match t {
      T::OpenParen => {
        self.dict.push_scope();
        self.pstk.push(ParseSt::Def(self.vstk.len()));
      },

      T::OpenBracket => self.pstk.push(ParseSt::List(self.vstk.len())),

      T::CloseBracket => { 
        Err(Error::CloseWithoutOpen)?
      },

      T::CloseParen => {
        let v = self.vstk.def(pos);
        self.vstk.push(V::F(v));
        self.dict.pop_scope();
        self.pstk.pop();
      },

      T::CloseDef(nm) => {
        let v = self.vstk.def(pos);
        self.dict.pop_scope();
        self.dict.insert(&nm,v);
        self.pstk.pop();
      },

      T::Z(v) => self.vstk.push(v),
      T::I(v) => self.vstk.push(v),
      T::True => self.vstk.push(true),
      T::False => self.vstk.push(false),

      T::Word(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::C(f.clone()));
      },

      T::QWord(nm) => {
        let f = self.dict.get(&nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()));
      },

      T::Str(s) => println!("str {s}")
    };

    Ok(())
  }
}

#[derive(Copy,Clone,Debug)]
enum ParseSt {
  List(usize),
  Def(usize)
}

#[derive(Debug,Copy,Clone)]
pub enum Error {
  Scanner(tokens::Error),
  CloseWithoutOpen,
  UnknownWord,
  Bif(bifs::Error)
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Scanner(e) => write!(f,"error while scanning tokens {e}"),
      Self::CloseWithoutOpen => write!(f,"Mismatched Close to a list or definition"),
      Self::UnknownWord => write!(f,"Unrecognized Word"),
      Self::Bif(e) => write!(f,"error in function call {e}"),
    }
  }
}

pub struct SrcError<'a> {
  src:&'a str,
  sp:std::ops::Range<usize>,
  etype:Error
}

impl<'a> std::fmt::Debug for SrcError<'a> {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    write!(f,"SrcError token \"{}\" type: {:?}",&self.src[self.sp.clone()],self.etype)
  }
}

impl<'a> std::fmt::Display for SrcError<'a> {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    write!(f,"Error on token {} type: {}",&self.src[self.sp.clone()],self.etype)
  }
}

impl<'a> std::error::Error for SrcError<'a> {}
