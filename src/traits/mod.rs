mod values;
pub use values::*;

pub trait Fort {
  type Extension:ExtensionType;
  type Environment;

  fn default_env() -> Self::Environment;
}
