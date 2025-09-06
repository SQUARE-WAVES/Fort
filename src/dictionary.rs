use std::{
  collections::HashMap,
  sync::Arc
};

use crate::{
  F,
  V,
  traits::Fort
};

#[derive(Default)]
pub struct Scope<S:Fort> {
  words:HashMap<Arc<str>,F<S>>
}

impl<S:Fort,T:Into<HashMap<Arc<str>,F<S>>>> From<T> for Scope<S> {
  fn from(wordbase:T) -> Self {
    Self{words:wordbase.into()}
  }
}

impl<S:Fort> Scope<S> {
  pub fn new() -> Self {
    Self {words:Default::default()}
  }

  pub fn lookup(&self,key:&str) -> Option<&F<S>> {
    self.words.get(key)
  }

  pub fn merge(mut self, other:Scope<S>) -> Scope<S> {
    for (k,v) in other.words {
      self.words.insert(k,v);
    }

    self
  }

  pub fn define<T1>(&mut self,vs:Arc<[V<S>]>,name:T1) -> F<S>
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

pub struct Dict<S:Fort> {
  root:Scope<S>,
  stk:Vec<Scope<S>>
}

impl<S:Fort> Dict<S> {
  pub fn new(root:Scope<S>) -> Self {
    Self {
      root,
      stk:vec![]
    }
  }

  pub fn push_scope(&mut self) {
    self.stk.push(Scope::<S>::new())
  }

  pub fn pop_scope(&mut self) {
    self.stk.pop();
  }

  pub fn drop_all_scopes(&mut self) {
    self.stk.clear();
  }

  pub fn lookup(&self,key:&str) -> Result<&F<S>,Error> {
    for scope in self.stk.iter().rev() {
      if let Some(f) = scope.lookup(key) { 
        return Ok(f) 
      }
    }

    self.root.lookup(key).ok_or(Error::UnknownWord)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V<S>]>,name:T1) -> F<S>
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


