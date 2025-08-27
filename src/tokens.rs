use logos::{Logos,Lexer};

#[derive(Debug,Copy,Clone,Logos)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
  #[token("(")]
  OpenParen,
  
  #[token(")")]
  CloseParen,
  
  #[token("[")]
  OpenBracket,
  
  #[token("]")]
  CloseBracket,

  #[regex(r"-?\d+\.\d+", |lx|lx.slice().parse().ok()) ]
  Z(f64),

  #[regex(r"-?\d+", |lx|lx.slice().parse().ok()) ]
  I(i64),

  #[regex(r"[^\(\)\[\]\d \t\n\f-][a-zA-Z0-9!@#$%^&*_<>|\+=]*")]
  Word(&'a str),

  #[regex(r"`[^\(\)\[\]\d \t\n\f-][a-zA-Z0-9!@#$%^&*_<>|\+=]*", |lx| &lx.slice()[1..])]
  QWord(&'a str),

  #[regex(r"\)::[^\(\)\[\]\d \t\n\f-][a-zA-Z0-9!@#$%^&*<>|_\+=]*", |lx| &lx.slice()[3..])]
  CloseDef(&'a str)
}

pub fn tokenize(s:&str) -> Lexer<Token>{
  Logos::lexer(s)
}


