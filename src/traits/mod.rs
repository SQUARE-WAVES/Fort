mod values;
pub use values::*;

pub trait Fort:Sized {
  type Extension:ExtensionType;
  type Environment;

  fn default_env() -> Self::Environment;
  fn dictionary() -> crate::Dict<Self>;
}
