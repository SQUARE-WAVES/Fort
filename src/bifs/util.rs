use super::{Vstack,Error};

pub fn print_stack(stk:&mut Vstack) -> Result<(),Error> {
  print!("\u{8} \u{8}");
  stk.print();
  println!();
  Ok(())
}
