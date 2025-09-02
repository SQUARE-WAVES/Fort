pub struct Vstack<V> {
  vs:Vec<V>
}

impl<V> Vstack<V> {
  pub fn push<T:Into<V>>(&mut self,t:T) {
    self.vs.push(t.into())
  }

  pub fn popv(&mut self) -> Option<V> {
    self.vs.pop()
  }

  pub fn pop<T>(&mut self) -> Option<Result<T,V::Error>> 
  where V:TryInto<T>
  {
    self.vs.pop().map(|v|v.try_into())
  }

  pub fn drain(&mut self,n:usize) -> impl Iterator<Item=V> {
    let start = self.vs.len().saturating_sub(n);
    self.vs.drain(start..)
  }

  pub fn clear(&mut self) {
    self.vs.clear()
  }

  pub fn peek(&self,amt:usize) -> &[V] {
    let start = self.vs.len().saturating_sub(amt);
    &self.vs[start..]
  }

  pub fn len(&self) -> usize {
    self.vs.len()
  }

  pub fn is_empty(&self) -> bool {
    self.vs.is_empty()
  }
}

//inherits
impl<V:std::fmt::Debug> std::fmt::Debug for Vstack<V> {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error>{
    write!(f,"{:?}",self.vs) 
  }
}

impl<V:std::fmt::Display> std::fmt::Display for Vstack<V> {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    if self.vs.is_empty() { return Ok(()) };

    let (first,rest) = &(self.vs[..]).split_at(1);

    write!(f,"{}",first[0])?;

    for v in rest.into_iter() {
      write!(f," {v}")?;
    };
    Ok(())
    
  }
}

impl<V> Default for Vstack<V> {
  fn default() -> Self { Self{vs:vec![]} }
}

impl<V:Clone> Clone for Vstack<V> {
  fn clone(&self) -> Self { 
    Self{vs:self.vs.clone()}
  }
}

impl<V> From<Vec<V>> for Vstack<V> {
  fn from(vs:Vec<V>) -> Self { Self{vs} }
}

impl<V> From<Vstack<V>> for Vec<V> {
  fn from(stk:Vstack<V>) -> Vec<V> { stk.vs }
}

impl<V> From<Vstack<V>> for std::sync::Arc<[V]> {
  fn from(stk:Vstack<V>) -> std::sync::Arc<[V]> { stk.vs.into() }
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
  fn test_pop() {
    let mut vs :Vstack<V> = Vstack::default();

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
    let vres = vs.pop::<f64>().expect("should get a return");
    let z = vres.expect("should get a float");
    assert_eq!(z,test_f);
  }
}
