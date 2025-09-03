use crate::{
  V,
  F,
  Vstack,
  Dict,
  traits::Fort
};

#[derive(Debug)]
pub enum Mode<S:Fort> {
  List(Vstack<V<S>>),
  Def(Vstack<V<S>>),
}

impl<S:Fort> Mode<S> {
  pub fn def() -> Self {
    Self::Def(Default::default())
  }

  pub fn list() -> Self {
    Self::List(Default::default())
  }

  pub fn vs(&mut self) -> &mut Vstack<V<S>> {
    match self {
      Self::List(stk) => stk,
      Self::Def(stk) => stk
    }
  }

  pub fn end(self) -> Vstack<V<S>> {
    match self {
      Self::List(stk) => stk,
      Self::Def(stk) => stk
    }
  }
}

impl<S:Fort> std::fmt::Display for Mode<S> {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::List(vs) => write!(f,"[{vs}"),
      Self::Def(vs) => write!(f,"({vs}")
    }
  }
}

pub struct Thread<'a,S:Fort> {
  root:Mode<S>,
  env:S::Environment,
  dict:&'a mut Dict<S>,
  mode_stack:Vec<Mode<S>>,
}

impl<'a,S:Fort> Thread<'a,S> {
  pub fn as_list(env:S::Environment,dict:&'a mut Dict<S>) -> Self {
    Self {
      root:Mode::list(),
      env,
      mode_stack:vec![],
      dict
    }
  }

  pub fn as_def(env:S::Environment,dict:&'a mut Dict<S>) -> Self {
    Self {
      root:Mode::list(),
      env,
      mode_stack:vec![],
      dict
    }
  }

  pub fn drop_modes(&mut self) {
    self.mode_stack.clear()
  }

  //basic debug printing type stuff
  pub fn print(&mut self) {
    if self.root.vs().is_empty() && self.mode_stack.is_empty() {
      println!("-=EMPTY=-");
    }
    else {
      print!("-= {} ",self.root.vs());
      for md in self.mode_stack.iter() {
        print!("{md}");
      }
      println!("=-");
    }
  }

  pub fn start_list(&mut self) { 
    self.mode_stack.push(Mode::list()) 
  }

  pub fn end_list(&mut self) -> Result<(),Error> {
    match self.mode_stack.pop() {
      Some(Mode::List(vs)) => {
        self.push_val(V::L(vs.into()));
        Ok(())
      },
      _ => {
        self.drop_modes();
        Err(Error::ListEnd)
      }
    }
  }

  pub fn start_def(&mut self) {
    self.dict.push_scope();
    self.mode_stack.push(Mode::def())
  }

  pub fn end_def(&mut self,name:Option<&str>) -> Result<(),Error> {
    let vs = match self.mode_stack.pop() {
      Some(Mode::Def(vs)) => {
        vs
      },
      _ => {
        self.drop_modes();
        return Err(Error::DefEnd)
      }
    };

    self.dict.pop_scope();

    let f = self.dict.define(vs.into(),name.map(|n|n.into()));

    if matches!(f,F::Anon(_)) {
      self.push_val(V::F(f))
    }

    Ok(())
  }

  pub fn push_val(&mut self,val:V<S>) {
    self.stk().push(val)
  }

  pub fn apply(&mut self,call:F<S>) -> Result<(),Error> {
    let md = self.mode();

    match md {
      Mode::Def(vs) => {
        vs.push(V::C(call));
        Ok(())
      },
      Mode::List(_) => call.run(self).map_err(Error::Call)
    }
  }

  pub fn lookup(&mut self,nm:&str) -> Result<F<S>,Error> {
    self.dict.get(nm).map_err(Error::Dictionary).cloned()
  }

  //this is shorthand
  pub fn word(&mut self,nm:&str) -> Result<(),Error> {
    let f = self.lookup(nm)?;
    self.apply(f)
  }

  pub fn quote(&mut self,nm:&str) -> Result<(),Error> {
    let f = self.lookup(nm)?;
    self.push_val(V::F(f));
    Ok(())
  }

  //ending stuff
  pub fn into_function(self) -> Result<F<S>,Error> {
    self.mode_stack.is_empty().then(||{
      F::Anon(self.root.end().into())
    })
    .ok_or(Error::NotDone)
  }

  //helpos
  pub fn stk(&mut self) -> &mut Vstack<V<S>> {
    self.mode_stack.last_mut().unwrap_or(&mut self.root).vs()
  }

  pub fn dict(&mut self) -> &mut Dict<S> {
    self.dict
  }

  fn mode(&mut self) -> &mut Mode<S> {
    self.mode_stack.last_mut().unwrap_or(&mut self.root)
  }

  pub fn env(&self) -> &S::Environment {
    &self.env
  }

  pub fn env_mut(&mut self) -> &mut S::Environment {
    &mut self.env
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
