use super::module;
use super::molecules;
use super::sample;

#[derive(Debug)]
pub struct Robot {
    location: module::Module,
    eta: u8,
    score: i16,
    inventory: molecules::Molecules,
    expertise: molecules::Molecules,
    held_samples: Vec<sample::Sample>,
}

impl Robot {
   const EXPERTISE_UNTIL_MIDDLE_RANK: i8 = 3;
   const EXPERTISE_UNTIL_HIGH_RANK: i8 = 9;
   pub fn get_held_samples(&self) -> &Vec<sample::Sample> {
      &self.held_samples
   }
   pub fn append_sample(&mut self, sample: sample::Sample) {
      self.held_samples.push(sample);
   }
   pub fn get_eta(&self) -> u8 {
      self.eta
   }
   pub fn get_location(&self) -> &module::Module {
      &self.location
   }
   pub fn new() -> Self {
      Self {
         location: module::Module::Spawn,
         eta: 0,
         score: 0,
         inventory: molecules::Molecules::new(),
         expertise: molecules::Molecules::new(),
         held_samples: Vec::new(),
      }
   }

   const MAX_SAMPLES: usize = 3;
   const MAX_MOLECULES: i8 = 10;
   pub fn new_from_inputs(inputs: Vec<&str>) -> Self {
      Self {
         location: module::Module::from_str(inputs[0].trim()).unwrap(),
         eta: parse_input!(inputs[1], u8),
         score: parse_input!(inputs[2], i16),
         inventory: molecules::Molecules::from_slice(&inputs[3..8]),
         expertise: molecules::Molecules::from_slice(&inputs[8..13]),
         held_samples: Vec::new(),
      }
   }

   pub fn get_unresearched_sample(&self) -> Option<&sample::Sample> {
      let filtered_values = self
         .held_samples
         .iter()
         .filter(|sample| match sample.get_health() {
            sample::SampleHealth::Unresearched => true,
            _ => false,
         })
         .collect::<Vec<_>>();

      if filtered_values.len() > 0 {
         Some(filtered_values[0])
      } else {
         None
      }
   }

   pub fn get_most_interesting_ready_sample(&self) -> Option<&sample::Sample> {
      let sorted_samples: Vec<&sample::Sample> = self.get_sorted_samples();
      for sample in sorted_samples {
         let cost = sample.get_cost() - &self.expertise;
         if self.inventory.has_enough(&cost.set_minues_to_zero()) {
            return Some(sample);
         } else {
            continue;
         }
      }
      return None;
   }

   pub fn has_maximum_samples(&self) -> bool {
      return self.held_samples.len() >= Self::MAX_SAMPLES;
   }

   pub fn has_maximum_molecules(&self) -> bool {
      return self.inventory.len() == Self::MAX_MOLECULES;
   }

   // TODO: inspect for similarities with has_enough_molecules and refactor
   pub fn pick_best_molecule(&self, available: &molecules::Molecules, ) -> Option<molecules::Molecule> {
      let sorted_samples: Vec<&sample::Sample> = self.get_sorted_samples();
      // go through every sample
      let mut held_molecules = self.inventory.clone();
      let mut additional_expertise = molecules::Molecules::new();
      for sample in sorted_samples {
         let needed_molecules = &(sample.get_cost() - &self.expertise) - &held_molecules;
         // check if already have everything needed
         if needed_molecules.is_not_positive() {
            held_molecules = &held_molecules - &(sample.get_cost() - &self.expertise).set_minues_to_zero();
            additional_expertise = &additional_expertise + sample.get_expertise_gain();
            continue;
         }
         if !available.has_enough(&needed_molecules.set_minues_to_zero()) {
            continue;
         }
         if needed_molecules.set_minues_to_zero().len() > (Self::MAX_MOLECULES - self.inventory.len()) {
            continue;
         }
         if let Some(found_molecule) = needed_molecules.get_next_molecule() {
            return Some(found_molecule);
         } else {
            continue;
         }
      }
      return None;
   }

   pub fn get_sorted_samples(&self) -> Vec<&sample::Sample> {
      let mut sorted_samples: Vec<&sample::Sample> =
         self.held_samples.iter().map(|sample| sample).collect();
      sorted_samples.sort_by_key(|&sample| match sample.get_health() {
         sample::SampleHealth::Researched(health) => -(*health as i8),
         _ => 0i8,
      });
      return sorted_samples;
   }

   pub fn has_enough_molecules(&self) -> bool {
      let sorted_samples: Vec<&sample::Sample> = self.get_sorted_samples();
      let mut held_molecules = self.inventory.clone();
      let mut accumulated_expertise = self.expertise.clone();
      for sample in sorted_samples {
         // copied from get_most_interesting_ready_sample
         let needed_molecules = sample.get_cost() - &(&accumulated_expertise + &held_molecules);
         if needed_molecules.is_not_positive() {
            accumulated_expertise = &accumulated_expertise + sample.get_expertise_gain();
            held_molecules = &held_molecules - &needed_molecules.set_minues_to_zero();
         } else {
            return false;
         }
      }
      return true;
   }

   pub fn can_produce_one_held_sample(&self, available: &molecules::Molecules) -> bool {
      self.held_samples
         .iter()
         .any(|sample| self.can_produce_sample(sample, available))
   }

   pub fn can_produce_sample(&self, sample: &sample::Sample, available: &molecules::Molecules) -> bool {
      let needed_molecules = sample.get_cost() - &(&self.expertise + &self.inventory);
      if needed_molecules.is_not_positive() {
         return true;
      }
      let remaining_required_molecules = needed_molecules.set_minues_to_zero();
      if remaining_required_molecules.len() + self.inventory.len() > Self::MAX_MOLECULES {
         return false;
      }
      available.has_enough(&remaining_required_molecules)
   }

   pub fn pick_sample_based_on_expertise(&self) -> sample::SampleRank {
      if self.expertise.len() < Self::EXPERTISE_UNTIL_MIDDLE_RANK {
         sample::SampleRank::LittleHealth
      } else if self.expertise.len() < Self::EXPERTISE_UNTIL_HIGH_RANK {
         sample::SampleRank::SomeHealth
      } else {
         sample::SampleRank::LotsOfHealth
      }
   }

   pub fn get_impossible_samples(&self, available: &molecules::Molecules) -> Vec<&sample::Sample> {
      self.get_held_samples()
         .iter()
         .filter(|sample: &&sample::Sample| !self.can_produce_sample(sample, available))
         .collect::<Vec<_>>()
   }

   pub fn has_enough_samples(&self) -> bool {
      self.held_samples.len() >= 2
   }
}
