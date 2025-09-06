use std::sync::Arc;

use crate::{
  V,
  F,
  FrameStack,
  Dict,
  traits::Fort
};

pub enum Mode {
  List,
  Def
}

pub struct Thread<'a,S:Fort> {
  env:S::Environment,
  dict:&'a mut Dict<S>,
  stk:FrameStack<V<S>,Mode>
}

impl<'a,S:Fort> Thread<'a,S> {
  pub fn new(dict:&'a mut Dict<S>,env:S::Environment) -> Self {
    Self{env,dict,stk:FrameStack::new()}
  }

  //the environment
  pub fn env(&self) -> &S::Environment {
    &self.env
  }

  pub fn env_mut(&mut self) -> &mut S::Environment {
    &mut self.env
  }

  //modes and frames
  pub fn drop_modes(&mut self) {
    self.dict.drop_all_scopes();
    self.stk.drop_all_frames();
  }

  pub fn start_list(&mut self) {
    self.stk.set_frame(Mode::List)
  }

  pub fn start_def(&mut self) {
    self.stk.set_frame(Mode::Def)
  }

  pub fn end_list(&mut self) -> Result<(),Error> {
    if let Some(Mode::List) = self.stk.mode() {
      let vs = self.stk.take_frame();
      let l = V::L(vs.into());
      self.stk.push(l);
      Ok(())
    }
    else {
      self.drop_modes();
      Err(Error::ListEnd)
    }
  }

  pub fn end_def(&mut self,nm:Option<&str>) -> Result<(),Error> {
    let f = if let Some(Mode::Def) = self.stk.mode() {
      let vs = self.stk.take_frame();
      self.dict.pop_scope();
      self.dict.define(vs.into(),nm.map(|s|s.into()))
    }
    else {
      self.drop_modes();
      return Err(Error::DefEnd)
    };

    if let F::Anon(_) = f {
      self.stk.push(f)
    }

    Ok(())
  }

  //for defined fns / tokens
  pub fn apply(&mut self,call:F<S>) -> Result<(),Error> {
    match self.stk.mode() {
      Some(Mode::Def) => {
        self.stk.push(V::C(call));
        Ok(())
      },
      _ => call.run(self).map_err(Error::Call)
    }
  }

  pub fn word(&mut self,txt:&str) -> Result<(),Error> {
    let f = self.lookup(txt)?;
    self.apply(f.clone())
  }

  pub fn quote(&mut self,txt:&str) -> Result<(),Error> {
    let f = self.lookup(txt)?;
    self.stk.push(V::F(f.clone()));
    Ok(())
  }

  //dictionary interface for bifs
  pub fn lookup(&self,key:&str) -> Result<&F<S>,Error> {
    self.dict.lookup(key).map_err(Error::Dictionary)
  }

  pub fn define<T1>(&mut self,vs:Arc<[V<S>]>,name:T1) -> F<S>
  where
    T1:Into<Option<Arc<str>>>
  {
    self.dict.define(vs,name)
  }

  //basic stack interface for bifs  
  pub fn push<T:Into<V<S>>>(&mut self,t:T) {
    self.stk.push(t.into())
  }

  pub fn popv(&mut self) -> Option<V<S>> {
    self.stk.popv()
  }

  pub fn pop<T>(&mut self) -> Option<Result<T,<V<S> as TryInto<T>>::Error>> 
  where V<S>:TryInto<T>
  {
    self.stk.pop()
  }

  pub fn dropn(&mut self,n:usize) {
    self.stk.dropn(n)
  }

  pub fn clear(&mut self) {
    self.stk.clear()
  }
  
  pub fn peek(&self,amt:usize) -> &[V<S>] {
    self.stk.peek(amt)
  }

  pub fn len(&self) -> usize {
    self.stk.len()
  }

  pub fn is_empty(&self) -> bool {
    self.stk.is_empty()
  }

  //debugging / repling
  pub fn print(&self) {
    let (pfx,sfx) = match self.stk.mode() {
      None => ("-=","=-"),
      Some(Mode::List) => ("-[","...-"),
      Some(Mode::Def) => ("-(","...-")
    };
    
    println!("{pfx} {} {sfx}",self.stk);
  }
}

#[derive(Clone,Copy,Debug)]
pub enum Error {
  Call(crate::bifs::Error),
  Dictionary(crate::dictionary::Error),
  DefEnd,
  ListEnd,
  NotDone
}

impl std::fmt::Display for Error {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Call(e) => write!(f,"Function Call Error: {e}"),
      Self::Dictionary(e) => write!(f,"Error looking up word: {e}"),
      Self::DefEnd => write!(f,"found the end of a function def when we weren't defining one"),
      Self::ListEnd => write!(f,"found the end of a List when we weren't making one"),
      Self::NotDone => write!(f,"Terminating VM while it's still making a list or def")
    }
  }
}

impl std::error::Error for Error {}
