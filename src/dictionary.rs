use std::{
  collections::HashMap,
  sync::Arc
};

use crate::{
  F,
  V,ExtType,
};

pub struct Scope<Ext:ExtType> {
  words:HashMap<Arc<str>,F<Ext>>
}

impl<Ext:ExtType> From<HashMap<Arc<str>,F<Ext>>> for Scope<Ext> {
  fn from(words:HashMap<Arc<str>,F<Ext>>) -> Self {
    Self{words}
  }
}

impl<Ext:ExtType> Scope<Ext> {
  pub fn new() -> Self {
    Self {words:Default::default()}
  }

  pub fn get(&self,key:&str) -> Option<&F<Ext>> {
    self.words.get(key)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V<Ext>]>,name:T1) -> F<Ext>
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

pub struct Dict<Ext:ExtType> {
  root:Scope<Ext>,
  stk:Vec<Scope<Ext>>
}

impl<Ext:ExtType> Dict<Ext> {
  pub fn new(root:Scope<Ext>) -> Self {
    Self {
      root,
      stk:vec![]
    }
  }

  pub fn push_scope(&mut self) {
    self.stk.push(Scope::<Ext>::new())
  }

  pub fn pop_scope(&mut self) {
    if self.stk.len() > 1 {
      self.stk.pop();
    }
  }

  pub fn get(&self,key:&str) -> Result<&F<Ext>,Error> {
    for scope in self.stk.iter().rev() {
      if let Some(f) = scope.get(key) { 
        return Ok(f) 
      }
    }

    self.root.get(key).ok_or(Error::UnknownWord)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V<Ext>]>,name:T1) -> F<Ext>
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


