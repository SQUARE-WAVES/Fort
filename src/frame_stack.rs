pub struct FrameStack<V,M> {
  vs:Vec<V>,
  frames:Vec<(M,usize)>
}

impl<V,M> FrameStack<V,M> {
  pub fn new() -> Self {
    Self{
      vs:vec![],
      frames:vec![]
    }
  }

  //internal stuff for frames
  fn frame(&self) -> usize {
    *self.frames.last().map(|(_,f)|f).unwrap_or(&0)
  }

  fn pop_frame(&mut self) -> usize {
    self.frames.pop().map(|(_,f)|f).unwrap_or(0)
  }

  fn frame_empty(&self) -> bool {
    self.vs.len() == self.frame()
  }

  //working with frames
  pub fn set_frame(&mut self,mode:M) {
    self.frames.push((mode,self.vs.len()))
  }

  /*
  pub fn drop_frame(&mut self) {
    let f = self.pop_frame();
    self.vs.truncate(f);
  }
  */

  //this drops everything except the root frame
  pub fn drop_all_frames(&mut self) {
    if let Some((_,f)) = self.frames.first() {
      self.vs.truncate(*f);
      self.frames.clear();
    }
  }

  pub fn take_frame(&mut self) -> Vec<V> {
    let f = self.pop_frame();
    self.vs.split_off(f)
  }

  pub fn peek_frame(&self) -> &[V] {
    &self.vs[self.frame()..]
  }

  pub fn mode(&self) -> Option<&M> {
    self.frames.last().map(|(m,_)|m)
  }

  //-----------------------------------------------------
  //the regular stack interface, but it only works on the top
  //frame
  pub fn push<T:Into<V>>(&mut self,t:T) {
    self.vs.push(t.into())
  }

  pub fn popv(&mut self) -> Option<V> {
    if !self.frame_empty() {
      self.vs.pop()
    }
    else {
      None
    }
  }

  pub fn pop<T>(&mut self) -> Option<Result<T,V::Error>> 
  where V:TryInto<T>
  {
    if !self.frame_empty() {
      self.vs.pop().map(|v|v.try_into())
    }
    else {
      None
    }
  }

  pub fn dropn(&mut self,n:usize) {
    let start = self.vs.len().saturating_sub(n).max(self.frame());
    self.vs.truncate(start)
  }

  pub fn clear(&mut self) {
    self.vs.truncate(self.frame())
  }
  
  pub fn peek(&self,amt:usize) -> &[V] {
    let start = self.vs.len().saturating_sub(amt).max(self.frame());
    &self.vs[start..]
  }

  pub fn len(&self) -> usize {
    self.vs.len() - self.frame()
  }

  pub fn is_empty(&self) -> bool {
    self.vs.len() == self.frame()
  }
}

//inherits
impl<V:std::fmt::Debug,M> std::fmt::Debug for FrameStack<V,M> {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error>{
    write!(f,"FrameStack< {:?} >",self.vs) 
  }
}

impl<V:std::fmt::Display,M> std::fmt::Display for FrameStack<V,M> {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    if self.vs.is_empty() { return Ok(()) };
    let (first,rest) = self.peek_frame().split_at(1);

    write!(f,"{}",first[0])?;

    for v in rest.iter() {
      write!(f," {v}")?;
    };
    Ok(())
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[derive(PartialEq,Debug)]
  pub enum V {
    I(i64),
    Z(f64)
  }

  impl From<i64> for V {
    fn from(n:i64) -> V {
      V::I(n)    
    }
  }
  
  impl From<f64> for V {
    fn from(n:f64) -> V {
      V::Z(n)    
    }
  }
  
  impl TryInto<i64> for V {
    type Error = V;
    fn try_into(self) -> Result<i64,Self::Error> {
      match self {
        V::I(n) => Ok(n),
        _ => Err(self)
      }
    }
  }
  
  impl TryInto<f64> for V {
    type Error = V;
    fn try_into(self) -> Result<f64,Self::Error> {
      match self {
        V::Z(n) => Ok(n),
        _ => Err(self)
      }
    }
  }

  #[test]
  fn test_push_pop() {
    let mut vs = FrameStack::<V,usize>::new();

    //push an int
    let test_n = 15;
    vs.push(test_n);
    assert_eq!(vs.len(),1);
    assert!(!vs.is_empty());

    //pop it back off
    let vres = vs.pop::<i64>().expect("a value should be returned");
    let v = vres.expect("value should convert");
    assert_eq!(v,test_n);
    assert_eq!(vs.len(),0);
    assert!(vs.is_empty());

    //try to pop off an empty stack
    let vres = vs.pop::<i64>();
    assert!(vres.is_none());

    let vres = vs.pop::<f64>();
    assert!(vres.is_none());

    let test_n = 22;
    vs.push(test_n);

    let vres = vs.pop::<f64>().expect("should get a return");
    let v = vres.expect_err("should be a conversion error");
    assert_eq!(V::I(test_n),v);

    let test_f = 2.3;
    vs.push(test_f);
    let vres = vs.pop::<f64>().expect("should get a return");
    let z = vres.expect("should get a float");
    assert_eq!(z,test_f);
  }

  #[test]
  fn test_frames() {
    let mut vs = FrameStack::<V,usize>::new();
    assert_eq!(vs.mode(),None);
    assert_eq!(vs.len(),0);
    assert!(vs.is_empty());

    //push some values
    vs.push(12);
    vs.push(2.2);
    assert_eq!(vs.len(),2);
    assert_eq!(&[V::I(12),V::Z(2.2)],vs.peek_frame());

    //push a frame
    vs.set_frame(10);
    assert_eq!(Some(&10),vs.mode());
    assert!(vs.is_empty());

    vs.push(5);
    assert_eq!(Some(Ok(5)),vs.pop::<i64>());
    assert_eq!(None,vs.pop::<i64>());
    assert_eq!(Some(&10),vs.mode());
    assert!(vs.is_empty());
    vs.drop_frame();

    assert_eq!(None,vs.mode());
    assert_eq!(vs.len(),2);
    assert_eq!(&[V::I(12),V::Z(2.2)],vs.peek_frame());
  }
}
