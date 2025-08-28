use std::collections::HashMap;
use crate::{F,bifs};

pub struct Dict {
  stk:Vec<HashMap<String,F>>
}

impl Default for Dict {
  fn default() -> Self {
    Self::new()
  }
}

impl Dict {
  pub fn new() -> Self {
    let root = bifs::root_dict();
    Self {
      stk:vec![root]
    }
  }

  pub fn push_scope(&mut self) {
    self.stk.push(HashMap::new())
  }

  pub fn pop_scope(&mut self) {
    if self.stk.len() > 1 {
      self.stk.pop();
    }
    else {
      panic!("popping root scope");
    }
  }

  pub fn get(&self,key:&str) -> Option<&F> {
    for scope in self.stk.iter().rev() {
      let v = scope.get(key);
      if v.is_some() { return v }
    }

    None
  }

  pub fn insert(&mut self,key:&str,val:F) {
    let scope = self.stk.last_mut().expect("somehow lost the dictionary");
    scope.insert(key.into(),val);
  }
}
