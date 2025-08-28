use crate::{
  V,
  Vstack,
  Dict,
  Token,
  tokenize,
};



#[derive(Default)]
pub struct VM {
  vstk:Vstack,
  pstk:Vec<ParseSt>,
  dict:Dict
}

impl VM {
  pub fn eval<'a>(&mut self,src:&'a str) -> Result<(),SrcError<'a>> {
    let tks = tokenize(src);
    for (tk,sp) in tks.spanned() {
      let tk = tk.map_err(|_|SrcError{
        src,
        sp:sp.clone(),
        etype:Error::UnknownToken
      })?;

      self.eat_token(tk).map_err(|etype|SrcError{src,sp,etype})?;
    }

    Ok(())
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
      T::Word(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        f.run(&mut self.vstk).map_err(|_|Error::BifError)?;
      },
      T::QWord(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()))
      }
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

      T::Z(v) => self.vstk.push(V::Z(v)),

      T::I(v) => self.vstk.push(V::I(v)),

      T::Word(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        f.run(&mut self.vstk).map_err(|_|Error::BifError)?;
      },

      T::QWord(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()))
      }
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
        self.dict.insert(nm,v);
        self.pstk.pop();
      },

      T::Z(v) => self.vstk.push(V::Z(v)),

      T::I(v) => self.vstk.push(V::I(v)),

      T::Word(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::C(f.clone()));
      },

      T::QWord(nm) => {
        let f = self.dict.get(nm).ok_or(Error::UnknownWord)?;
        self.vstk.push(V::F(f.clone()));
      }
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
  UnknownToken,
  CloseWithoutOpen,
  UnknownWord,
  BifError
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::UnknownToken => write!(f,"Unrecognized Token"),
      Self::CloseWithoutOpen => write!(f,"Mismatched Close to a list or definition"),
      Self::UnknownWord => write!(f,"Unrecognized Word"),
      Self::BifError => write!(f,"an error in a function call")
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





