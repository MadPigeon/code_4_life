#[derive(Debug, PartialEq)]
pub enum Module {
   Sample,
   Diagnosis,
   Molecule,
   Laboratory,
   Spawn,
}

impl Module {
   const SAMPLE: &str = "SAMPLES";
   const DIAGNOSIS: &str = "DIAGNOSIS";
   const MOLECULES: &str = "MOLECULES";
   const LABORATORY: &str = "LABORATORY";
   const SPAWN: &str = "START_POS";

   pub fn from_str(s: &str) -> Result<Self, &'static str> {
      match s {
         Self::SAMPLE => Ok(Module::Sample),
         Self::DIAGNOSIS => Ok(Module::Diagnosis),
         Self::MOLECULES => Ok(Module::Molecule),
         Self::LABORATORY => Ok(Module::Laboratory),
         Self::SPAWN => Ok(Module::Spawn),
         _ => Err("Invalid module name"),
      }
   }

   pub fn as_str(&self) -> &str {
      match self {
         Module::Spawn => Self::SPAWN,
         Module::Sample => Self::SAMPLE,
         Module::Diagnosis => Self::DIAGNOSIS,
         Module::Molecule => Self::MOLECULES,
         Module::Laboratory => Self::LABORATORY,
      }
   }
}
