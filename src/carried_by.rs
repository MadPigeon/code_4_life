#[derive(Debug, PartialEq)]
pub enum CarriedBy {
   Me = 0,
   Other = 1,
   Cloud = -1,
}

impl CarriedBy {
   pub fn from_integer(value: i8) -> Option<Self> {
      match value {
         0 => Some(CarriedBy::Me),
         1 => Some(CarriedBy::Other),
         -1 => Some(CarriedBy::Cloud),
         _ => None,
      }
   }
}
