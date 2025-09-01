use std::{
  collections::HashMap,
  sync::Arc
};

use crate::{F,V,bifs};

#[derive(Default)]
pub struct Scope {
  words:HashMap<Arc<str>,F>
}

impl From<HashMap<Arc<str>,F>> for Scope {
  fn from(words:HashMap<Arc<str>,F>) -> Self {
    Self{words}
  }
}

impl Scope {
  pub fn get(&self,key:&str) -> Option<&F> {
    self.words.get(key)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V]>,name:T1) -> F
  where
    T1:Into<Option<Arc<str>>>,
  {
    match name.into() {
      None => F::Anon(vs),
      Some(nm) => {
        let f = F::Def(nm.clone(),vs);
        self.words.insert(nm,f.clone());
        f
      }
    }
  }
}

pub struct Dict {
  root:Scope,
  stk:Vec<Scope>
}

impl Default for Dict {
  fn default() -> Self {
    Self::new()
  }
}

impl Dict {
  pub fn new() -> Self {
    let root :Scope = bifs::root_dict().into();
    Self {
      root,
      stk:vec![]
    }
  }

  pub fn push_scope(&mut self) {
    self.stk.push(Scope::default())
  }

  pub fn pop_scope(&mut self) {
    if self.stk.len() > 1 {
      self.stk.pop();
    }
  }

  pub fn get(&self,key:&str) -> Result<&F,Error> {
    for scope in self.stk.iter().rev() {
      if let Some(f) = scope.get(key) { 
        return Ok(f) 
      }
    }

    self.root.get(key).ok_or(Error::UnknownWord)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V]>,name:T1) -> F
  where
    T1:Into<Option<Arc<str>>>
  {
    let scope = self.stk.last_mut().unwrap_or(&mut self.root);
    scope.define(vs,name)
  }
}

#[derive(Copy,Clone,Debug)]
pub enum Error {
  UnknownWord,
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::UnknownWord => write!(f,"unknown word"),
    }
  }
}


