use super::molecules;

#[derive(Debug)]
pub enum SampleRank {
   LotsOfHealth = 3,
   SomeHealth = 2,
   LittleHealth = 1,
}

#[derive(Debug)]
pub enum SampleHealth {
   Unresearched,
   Researched(u8),
}

#[derive(Debug)]
pub struct Sample {
   id: u8,
   rank: SampleRank,
   health: SampleHealth,
   cost: molecules::Molecules,
   expertise_gain: molecules::Molecules,
}

impl Sample {
   pub fn get_health(&self) -> &SampleHealth {
      &self.health
   }
   pub fn get_cost(&self) -> &molecules::Molecules {
      &self.cost
   }
   pub fn get_expertise_gain(&self) -> &molecules::Molecules {
      &self.expertise_gain
   }
   pub fn get_id(&self) -> u8 {
      self.id
   }
   pub fn new(
      id: u8,
      rank: SampleRank,
      health: SampleHealth,
      cost: molecules::Molecules,
      expertise_gain: molecules::Molecules,
   ) -> Self {
      Self {
         id,
         rank,
         health,
         cost,
         expertise_gain,
      }
   }
}

impl SampleHealth {
   pub fn from_integer(number: i8) -> Self {
      if number < 0 {
         SampleHealth::Unresearched
      } else {
         SampleHealth::Researched(number as u8)
      }
   }
}

impl SampleRank {
   pub fn as_value(&self) -> u8 {
      match self {
         SampleRank::LotsOfHealth => 3,
         SampleRank::SomeHealth => 2,
         SampleRank::LittleHealth => 1,
      }
   }

   pub fn from_integer(value: i8) -> Option<Self> {
      match value {
         3 => Some(SampleRank::LotsOfHealth),
         2 => Some(SampleRank::SomeHealth),
         1 => Some(SampleRank::LittleHealth),
         _ => None,
      }
   }
}
