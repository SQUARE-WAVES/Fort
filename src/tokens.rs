use logos::{Logos,Lexer};
use crate::{V,Vstack,dictionary::Dict};

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

#[derive(Copy,Clone,Debug)]
pub enum ParseState {
  List(usize),
  Def(usize)
}

fn base_token(t:Token,stk:&mut Vstack,dict:&mut Dict,pstk:&mut Vec<ParseState>) {
  use Token as T;

  match t {
    T::OpenParen => {
      dict.push_scope();
      pstk.push(ParseState::Def(stk.len()))
    }
    T::OpenBracket => pstk.push(ParseState::List(stk.len())),
    T::CloseParen | T::CloseDef(_)| T::CloseBracket => panic!("close without open"),
    T::Z(v) => stk.push(V::Z(v)),
    T::I(v) => stk.push(V::I(v)),
    T::Word(nm) => {
      let f = dict.get(nm).expect("unkown word");
      f.run(stk)
    },
    T::QWord(nm) => {
      let f = dict.get(nm).expect("unknown word quoted");
      stk.push(V::F(f.clone()))
    }
  };
}

fn list_token(t:Token,stk:&mut Vstack,dict:&mut Dict,lpos:usize,pstk:&mut Vec<ParseState>) {
  use Token as T;

  match t {
    T::OpenParen => {
      dict.push_scope();
      pstk.push(ParseState::Def(stk.len()))
    },
    T::OpenBracket => pstk.push(ParseState::List(stk.len())),
    T::CloseBracket => {
      let v = stk.lst(lpos);
      stk.push(v);
      pstk.pop();
    }
    T::CloseParen | T::CloseDef(_) => panic!("close without open"),
    T::Z(v) => stk.push(V::Z(v)),
    T::I(v) => stk.push(V::I(v)),
    T::Word(nm) => {
      let f = dict.get(nm).expect("unkown word");
      f.run(stk)
    },
    T::QWord(nm) => {
      let f = dict.get(nm).expect("unknown word quoted");
      stk.push(V::F(f.clone()))
    }
  };
}

fn def_token(t:Token,stk:&mut Vstack,dict:&mut Dict,lpos:usize,pstk:&mut Vec<ParseState>) {
  use Token as T;

  match t {
    T::OpenParen => {
      dict.push_scope();
      pstk.push(ParseState::Def(stk.len()));
    }
    T::OpenBracket => pstk.push(ParseState::List(stk.len())),
    T::CloseBracket => panic!("close without open"),
    T::CloseParen => {
      let v = stk.def(lpos);
      stk.push(V::F(v));
      dict.pop_scope();
      pstk.pop();
    },
    T::CloseDef(nm) => {
      let v = stk.def(lpos);
      dict.pop_scope();
      dict.insert(nm,v);
      pstk.pop();
    }
    T::Z(v) => stk.push(V::Z(v)),
    T::I(v) => stk.push(V::I(v)),
    T::Word(nm) => {
      let f = dict.get(nm).expect("unkown word");
      stk.push(V::C(f.clone()));
    },
    T::QWord(nm) => {
      let f = dict.get(nm).expect("unkown word");
      stk.push(V::F(f.clone()));
    }
  };
}

pub fn eat_token(t:Token,stk:&mut Vstack,dict:&mut Dict,pstack:&mut Vec<ParseState>) {
  match pstack.last() {
    Some(ParseState::List(n)) => list_token(t,stk,dict,*n,pstack),
    Some(ParseState::Def(n)) => def_token(t,stk,dict,*n,pstack),
    None => base_token(t,stk,dict,pstack)
  }
}
