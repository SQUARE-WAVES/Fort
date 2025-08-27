use std::collections::HashMap;

mod vstack;
use vstack::*;
mod tokens;
use tokens::*;
mod dictionary;
use dictionary::*;
mod bifs;

fn main() {
  let mut vstk = Vstack::default();
  let mut pstack : Vec<ParseState> = vec![];
  let mut dict = Dict::new();
  let src = "((1 +)::inc inc inc)::dub (`dub map)::up_it [1 2 3] up_it"; 
  let tks = tokenize(src);

  for tk in tks {

    let tk = tk.expect("bad token");
    eat_token(tk,&mut vstk,&mut dict,&mut pstack);
    vstk.print();
  }
}
