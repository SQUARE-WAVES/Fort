#[derive(Debug)]
pub enum Token<'a> {
  OpenParen,
  CloseParen,
  OpenBracket,
  CloseBracket,
  True,False,
  Word(&'a str),
  CloseDef(&'a str),
  QWord(&'a str),
  I(i64),
  Z(f64),
  Str(&'a str)

  //SymWord(&'a str),
}

#[derive(Default,Debug,PartialEq,Clone,Copy)]
pub enum St {
  #[default]
  Base,
  Minus,
  Word,
  Qw,
  Num,
  Float,
  Str,
  CloseP,
  CloseDef
}

pub struct Scanner<'a> {
  src:&'a str,
  pos:usize,
  comp:usize,
  state:St
}

impl<'a> Scanner<'a> {
  pub fn resume(src:&'a str,pos:usize,state:St) -> Self {
    Self{src,pos,comp:0,state}
  }

  pub fn done(self) -> (St,usize,usize) {
    (self.state,self.comp,self.pos-self.comp)
  }

  fn ch(&mut self) -> Option<char> {
    let c = &self.src[self.pos..].chars().next()?;
    Some(*c)
  }

  fn step(&mut self,c:char) {
    self.pos += c.len_utf8()
  }

  fn clear(&mut self) {
    self.comp = self.pos
  }

  //I gotta do it like this cause of borrow checker
  //stuff
  fn seen(&self) -> &'a str {
    let (_,rest) = self.src.split_at(self.comp);
    let(s,_) = rest.split_at(self.pos-self.comp);
    s.trim_start()
  }

  fn complete(&mut self,tk:Token<'a>) -> Token<'a> {
    self.clear();
    self.state=St::Base;
    tk
  }

  fn advance(&mut self,c:char,st:St) -> Result<Token,Error> {
    self.step(c);
    self.state=st;
    self.eat()
  }
 
  pub fn eat(&mut self) -> Result<Token,Error> { 
    let c = self.ch().ok_or(Error::Done)?;

    match self.state {
      St::Base => self.base_state(c),
      St::Word => self.word_state(c),
      St::Qw => self.qw_state(c),
      St::Num => self.num_state(c),
      St::Float => self.float_state(c),
      St::Str => self.str_state(c),
      St::CloseP => self.close_p_state(c),
      St::CloseDef => self.close_def_state(c),
      St::Minus => self.minus_state(c),
    }
  }

  fn base_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      '(' => {
        self.step(c);
        Ok(self.complete(Token::OpenParen))
      },
      ')' => self.advance(c,St::CloseP),
      '[' => {
        self.step(c);
        Ok(self.complete(Token::OpenBracket))
      },
      ']' => { 
        self.step(c);
        Ok(self.complete(Token::CloseBracket))
      },
      '-' => self.advance(c,St::Minus),
      '"' => self.advance(c,St::Str),
      '`' => self.advance(c,St::Qw),
      _ if c.is_alphabetic() || word_start(c) => self.advance(c,St::Word),
      _ if c.is_numeric() =>  self.advance(c,St::Num),
      _ if c.is_whitespace() => self.advance(c,St::Base),
      _ => Err(Error::InvalidChar)
    }
  }

  fn qw_state(&mut self,c:char) -> Result<Token,Error> {
    if word_start(c) {
      self.advance(c,St::Word)
    }
    else {
      Err(Error::InvalidCharInWord)
    }
  }
  
  fn close_p_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      ':' => self.advance(c,St::CloseDef),
      _ => Ok(self.complete(Token::CloseParen))
    }
  }

  fn close_def_state(&mut self,c:char) -> Result<Token,Error>{
    let work = self.seen();
    match (&work[0..work.len().min(3)],c) {
      ("):",':') => self.advance(c,St::CloseDef),
      (")::",_) if word_start(c) => {
        self.advance(c,St::Word)
      },
      _ => Err(Error::InvalidDefClose) 
    }
  }

  fn word_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      _ if word_char(c) => self.advance(c,St::Word),
      '(' | ')' | '[' | ']' => self.word_terminal(),
      _ if c.is_whitespace()  => self.word_terminal(),
      _ => Err(Error::InvalidCharInWord) 
    }
  }

  fn word_terminal(&mut self) -> Result<Token,Error>{
    let work = self.seen();

    let tk = if let Some(w) = work.strip_prefix(")::") {
      Token::CloseDef(w)
    }
    else if let Some(w) = work.strip_prefix("`") {
      Token::QWord(w)
    }
    else if work == "true" {
      Token::True
    }
    else if work == "false" {
      Token::False
    }
    else {
      Token::Word(work)
    };

    let out = self.complete(tk);
    Ok(out)
  }

  fn minus_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      '(' | ')' | '[' | ']' => Ok(self.complete(Token::Word(self.seen()))),
      _ if c.is_whitespace() => Ok(self.complete(Token::Word(self.seen()))),
      _ if c.is_numeric() => self.advance(c,St::Num),
      _ if word_start(c) => self.advance(c,St::Word),
      _ => Err(Error::InvalidCharAfterMinus)
    }
  }

  fn num_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      '.' => self.advance(c,St::Float),
      _ if c.is_numeric() => self.advance(c,St::Num),
      _ if c.is_whitespace() => self.int_terminal(),
      '(' | ')' | '[' | ']' => self.int_terminal(),
      _ => Err(Error::InvalidCharInNumber)
    }
  }

  fn int_terminal(&mut self) -> Result<Token,Error> {
    let work = self.seen();
    work.parse::<i64>()
    .map(|n|self.complete(Token::I(n)))
    .map_err(|_|Error::UnparseableInt)
  }

  fn float_state(&mut self,c:char) -> Result<Token,Error> {
    match c {
      _ if c.is_numeric() => self.advance(c,St::Float),
      '(' | ')' |'[' | ']' => self.float_terminal(),
      _ if c.is_whitespace() => self.float_terminal(),
      _ => Err(Error::InvalidCharInNumber)
    }
  }

  fn float_terminal(&mut self) -> Result<Token,Error> {
    let work = self.seen();
    work.parse::<f64>()
    .map(|n|self.complete(Token::Z(n)))
    .map_err(|_|Error::UnparseableFloat)
  }

  fn str_state(&mut self,c:char) -> Result<Token,Error> {
    let work = self.seen();
    match c {
      '"' if !work.ends_with('\\') => {
        self.step(c);
        let tk = Token::Str(&work[1..]);
        Ok(self.complete(tk))
      }
      _ => self.advance(c,St::Str)
    }
  }
}

//some helpers
fn word_start(c:char) -> bool {
  c.is_alphabetic() || "/!@#$%^&*_+?|<>,.".contains(c)
}

fn word_char(c:char) -> bool {
  c.is_alphanumeric() || "/!@#$%^&*_+?|<>,.-_".contains(c) 
}

#[derive(Debug,Clone,Copy)]
pub enum Error {
  Done,
  InvalidChar,
  InvalidCharInWord,
  InvalidCharInNumber,
  InvalidDefClose,
  UnparseableInt,
  UnparseableFloat,
  InvalidCharAfterMinus
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Done => write!(f,"no more input"),
      Self::InvalidChar => write!(f,"ran into a bad char"),
      Self::InvalidCharInWord => write!(f,"ran into a bad char while reading a word"),
      Self::InvalidCharInNumber => write!(f,"ran into a bad char while reading a number"),
      Self::InvalidDefClose => write!(f,"ran into something weird in a fn definition close"),
      Self::UnparseableInt => write!(f,"found an int but we couldn't parse it to an i64"),
      Self::UnparseableFloat => write!(f,"found a float but we couln't parse it to an f64"),
      Self::InvalidCharAfterMinus => write!(f,"found something after a minus that isn't good")
    }
  }
}
