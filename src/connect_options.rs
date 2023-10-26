use super::molecules;
use super::sample;

#[derive(Debug)]
pub enum ConnectOptions {
   SampleId(u8),
   SampleRank(sample::SampleRank),
   MoleculeType(molecules::Molecule),
}

impl ConnectOptions {
   pub fn to_string(&self) -> String {
      match self {
         ConnectOptions::SampleRank(rank) => rank.as_value().to_string(),
         ConnectOptions::MoleculeType(molecule_type) => molecule_type.as_char().to_string(),
         ConnectOptions::SampleId(id) => id.to_string(),
      }
   }
}
