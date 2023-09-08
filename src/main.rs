/**
 * Bring data on patient samples from the diagnosis machine to the laboratory
 * with enough molecules to produce medicine!
 **/

/**
 * 1. Collect sample data at the SAMPLES module
 * 2. Analyze them at the DIAGNOSIS module
 * 3. Gather required molecules for the medicines at the MOLECULES module
 * 4. Produce the medicines at the LABORATORY modue
 * 
 * can carry up to 3 sample data files and 10 molecules
 * molecule types = A,B,C,D,E
 */
use std::ops::{Add, Sub};
use std::io;

macro_rules! parse_input {
   ($x:expr, $t:ty) => {
      $x.trim().parse::<$t>().unwrap()
   };
}

#[derive(Debug)]
enum GameGoals {
   TakeSamples,
   ResearchSamples,
   GatherMolecules,
   ProduceMedicine,
   DropSamples,
}

#[derive(Debug, PartialEq)]
enum Module {
   Sample,
   Diagnosis,
   Molecule,
   Laboratory,
   Spawn,
}

#[derive(Debug, PartialEq)]
enum CarriedBy {
   Me = 0,
   Other = 1,
   Cloud = -1,
}

#[derive(Debug)]
enum SampleRank {
   LotsOfHealth = 3,
   SomeHealth = 2,
   LittleHealth = 1,
}

#[derive(Debug)]
enum ConnectOptions {
   SampleId(u8),
   SampleRank(SampleRank),
   MoleculeType(Molecule),
}

#[derive(Debug)]
enum Command {
   Goto(Module),
   Connect(ConnectOptions),
   Wait,
}

#[derive(Debug)]
enum Molecule {
   A,B,C,D,E
}

#[derive(Clone, Debug)]
struct Molecules {
   a: i8,
   b: i8,
   c: i8,
   d: i8,
   e: i8,
}

#[derive(Debug)]
struct Sample {
   id: u8,
   rank: SampleRank,
   health: SampleHealth,
   cost: Molecules,
   expertise_gain: Molecules
}

#[derive(Debug)]
enum SampleHealth {
   Unresearched,
   Researched(u8)
}

#[derive(Debug)]
struct Memory {
   goal: GameGoals,
   projects: Vec<Molecules>,
   my_robot: Robot,
   enemy_robot: Robot,
   available: Molecules,
   cloud: Vec<Sample>,
}

#[derive(Debug)]
struct Robot {
   location: Module,
   eta: u8,
   score: i16,
   inventory: Molecules,
   expertise: Molecules,
   held_samples: Vec<Sample>
}

impl Molecule {
   fn as_char(&self) -> char {
      match self {
         Molecule::A => 'A',
         Molecule::B => 'B',
         Molecule::C => 'C',
         Molecule::D => 'D',
         Molecule::E => 'E',
      }
   }
}

impl SampleRank {
   fn as_value(&self) -> u8 {
       match self {
           SampleRank::LotsOfHealth => 3,
           SampleRank::SomeHealth => 2,
           SampleRank::LittleHealth => 1,
       }
   }

   fn from_integer(value: i8) -> Option<Self> {
      match value {
         3 => Some(SampleRank::LotsOfHealth),
         2 => Some(SampleRank::SomeHealth),
         1 => Some(SampleRank::LittleHealth),
         _ => None
      }
   }
}

impl ConnectOptions {
   fn to_string(&self) -> String {
      match self {
         ConnectOptions::SampleRank(rank) => rank.as_value().to_string(),
         ConnectOptions::MoleculeType(molecule_type) => molecule_type.as_char().to_string(),
         ConnectOptions::SampleId(id) => id.to_string(),
      }
   }
}

impl Command {
   fn to_string(&self) -> String {
      match self {
         Command::Goto(module) => format!("GOTO {}", module.as_str()),
         Command::Connect(options) => format!("CONNECT {}", options.to_string()),
         _ => "WAIT".to_owned(),
      }
   }
}

impl CarriedBy {
   fn from_integer(value: i8) -> Option<Self> {
      match value {
         0 => Some(CarriedBy::Me),
         1 => Some(CarriedBy::Other),
         -1 => Some(CarriedBy::Cloud),
         _ => None
      }
   }
}

impl SampleHealth {
   fn from_integer(number: i8) -> Self {
      if number < 0 {
         SampleHealth::Unresearched
      } else {
         SampleHealth::Researched(number as u8)
      }
   }
}

impl Molecules {
   const MIN_CONSTRUCTOR_SLICE_LENGTH: usize = 5;
   pub fn new() -> Self {
      Self {
         a: 0,
         b: 0,
         c: 0,
         d: 0,
         e: 0,
      }
   }

   pub fn from_slice(slice: &[&str]) -> Self {
      if slice.len() >= Self::MIN_CONSTRUCTOR_SLICE_LENGTH {
         Self {
            a: parse_input!(slice[0], i8),
            b: parse_input!(slice[1], i8),
            c: parse_input!(slice[2], i8),
            d: parse_input!(slice[3], i8),
            e: parse_input!(slice[4], i8),
         }
      } else {
         panic!("Tried reading molecule from a short slice");
      }
   }

   pub fn len(&self) -> i8 {
      self.a + self.b + self.c + self.d + self.e
   }

   pub fn is_not_positive(&self) -> bool {
      self.a <= 0 && self.b <= 0 && self.c <= 0 && self.d <= 0 && self.e <= 0
   }

   pub fn from_letter(letter: char) -> Self {
      let mut molecules = Molecules::new();
      match letter {
         'A' => molecules.a = 1,
         'B' => molecules.b = 1,
         'C' => molecules.c = 1,
         'D' => molecules.d = 1,
         'E' => molecules.e = 1,
         _ => {}
      }
      molecules
   }

   fn set_minues_to_zero(&self) -> Molecules {
      let mut new_non_zero = self.clone();
      if new_non_zero.a < 0 {
         new_non_zero.a = 0;
      }
      if new_non_zero.b < 0 {
         new_non_zero.b = 0;
      }
      if new_non_zero.c < 0 {
         new_non_zero.c = 0;
      }
      if new_non_zero.d < 0 {
         new_non_zero.d = 0;
      }
      if new_non_zero.e < 0 {
         new_non_zero.e = 0;
      }
      new_non_zero
   }

   fn has_enough(&self, required: &Molecules) -> bool {
      if self.a - required.a < 0 {
         return false;
      }
      if self.b - required.b < 0 {
         return false;
      }
      if self.c - required.c < 0 {
         return false;
      }
      if self.d - required.d < 0 {
         return false;
      }
      if self.e - required.e < 0 {
         return false;
      }
      true
   }

   fn get_next_molecule(&self) -> Option<Molecule> {
      if self.a > 0 {
         return Some(Molecule::A);
      }
      if self.b > 0 {
         return Some(Molecule::B)
      }
      if self.c > 0 {
         return Some(Molecule::C);
      }
      if self.d > 0 {
         return Some(Molecule::D);
      }
      if self.e > 0 {
         return Some(Molecule::E);
      }
      return None;
   }
}

impl Robot {
   const EXPERTISE_UNTIL_MIDDLE_RANK: i8 = 3;
   const EXPERTISE_UNTIL_HIGH_RANK: i8 = 9;
   pub fn new() -> Self {
      Self {
         location: Module::Spawn,
         eta: 0,
         score: 0,
         inventory: Molecules::new(),
         expertise: Molecules::new(),
         held_samples: Vec::new()
      }
   }

   const MAX_SAMPLES: usize = 3;
   const MAX_MOLECULES: i8 = 10;
   fn set_from_inputs(&mut self, inputs: Vec<&str>) {
      self.location = Module::from_str(inputs[0].trim()).unwrap();
      self.eta = parse_input!(inputs[1], u8);
      self.score = parse_input!(inputs[2], i16);
      self.inventory = Molecules::from_slice(&inputs[3..8]);
      self.expertise = Molecules::from_slice(&inputs[8..13]);
   }

   fn get_unresearched_sample(&self) -> Option<&Sample> {
      let filtered_values = self.held_samples.iter()
         .filter(|sample| match sample.health {
            SampleHealth::Unresearched => true,
            _ => false
         }).collect::<Vec<_>>();

         if filtered_values.len() > 0 {
            Some(filtered_values[0])
         } else {
            None
         }
   }

   fn get_most_interesting_ready_sample(&self) -> Option<&Sample> {
      let sorted_samples: Vec<&Sample> = self.get_sorted_samples();
      for sample in sorted_samples {
         let cost = sample.cost.clone() - self.expertise.clone();
         if self.inventory.has_enough(&cost.set_minues_to_zero()) {
            return Some(sample);
         } else {
            continue;
         }
      }
      return None;
   }

   fn has_maximum_samples(&self) -> bool {
      return self.held_samples.len() >= Self::MAX_SAMPLES
   }

   fn has_maximum_molecules(&self) -> bool {
      return self.inventory.len() == Self::MAX_MOLECULES;
   }

   fn pick_best_molecule(&self, available: &Molecules) -> Option<Molecule> {
      let sorted_samples: Vec<&Sample> = self.get_sorted_samples();
      // go through every sample
      let mut held_molecules = self.inventory.clone();
      let mut additional_expertise = Molecules::new();
      for sample in sorted_samples {
         let needed_molecules = sample.cost.clone() - self.expertise.clone() - held_molecules.clone();
         // check if already have everything needed
         if needed_molecules.is_not_positive() {
            held_molecules = held_molecules - (sample.cost.clone() - self.expertise.clone()).set_minues_to_zero();
            additional_expertise = additional_expertise + sample.expertise_gain.clone();
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

   fn get_sorted_samples(&self) -> Vec<&Sample> {
      let mut sorted_samples: Vec<&Sample> = self.held_samples.iter()
         .map(|sample| sample).collect();
      sorted_samples.sort_by_key(|&sample| match sample.health {
         SampleHealth::Researched(health) => -(health as i8),
         _ => 0i8
      });
      return sorted_samples;
   }

   fn has_enough_molecules(&self) -> bool {
      let sorted_samples: Vec<&Sample> = self.get_sorted_samples();
      let mut held_molecules = self.inventory.clone();
      let mut accumulated_expertise = self.expertise.clone();
      for sample in sorted_samples {
         // copied from get_most_interesting_ready_sample
         let needed_molecules = sample.cost.clone() - accumulated_expertise.clone() - held_molecules.clone();
         if needed_molecules.is_not_positive() {
            accumulated_expertise = accumulated_expertise + sample.expertise_gain.clone();
            held_molecules = held_molecules - needed_molecules.set_minues_to_zero();
         } else {
            return false;
         }
      }
      return true;
   }

   fn can_produce_held_samples(&self, available: &Molecules) -> bool {
      for sample in &self.held_samples {
         let needed_molecules = sample.cost.clone() - self.expertise.clone() - self.inventory.clone();
         if needed_molecules.is_not_positive() {
            return true;
         }
         let remaining_required_molecules = needed_molecules.set_minues_to_zero();
         if remaining_required_molecules.len() + self.inventory.len() > Self::MAX_MOLECULES {
            continue;
         }
         if available.has_enough(&remaining_required_molecules) {
            return true;
         }
      }
      return false;
   }

   fn pick_sample_based_on_expertise(&self) -> SampleRank {
      if self.expertise.len() < Self::EXPERTISE_UNTIL_MIDDLE_RANK {
         SampleRank::LittleHealth
      } else if self.expertise.len() < Self::EXPERTISE_UNTIL_HIGH_RANK {
         SampleRank::SomeHealth
      } else {
         SampleRank::LotsOfHealth
      }
   }
}

impl Memory {
   pub fn new() -> Self {
      Self {
         goal: GameGoals::TakeSamples,
         projects: Vec::new(),
         my_robot: Robot::new(),
         enemy_robot: Robot::new(),
         available: Molecules::new(),
         cloud: Vec::new(),
      }
   }

   pub fn parse_initial_input(&mut self) {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      let project_count = parse_input!(input_line, u8);

      self.projects = Vec::new();
      for _ in 0..project_count {
         io::stdin().read_line(&mut input_line).unwrap();
         let inputs = input_line.split_whitespace().collect::<Vec<_>>();
         self.projects.push(Molecules::from_slice(&inputs[0..5]));
      }
   }

   pub fn parse_turn_input(&mut self) {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      self.my_robot
         .set_from_inputs(input_line.split_whitespace().collect::<Vec<_>>());

      input_line.clear();
      io::stdin().read_line(&mut input_line).unwrap();
      self.enemy_robot
         .set_from_inputs(input_line.split_whitespace().collect::<Vec<_>>());

      input_line.clear();
      io::stdin().read_line(&mut input_line).unwrap();
      let inputs = input_line.split_whitespace().collect::<Vec<_>>();
      self.available = Molecules::from_slice(&inputs[0..5]);

      self.cloud = Vec::new();
      self.my_robot.held_samples = Vec::new();
      self.enemy_robot.held_samples = Vec::new();
      
      input_line.clear();
      io::stdin().read_line(&mut input_line).unwrap();
      let sample_count = parse_input!(input_line, u16);
      for _ in 0..sample_count {
         input_line.clear();
         io::stdin().read_line(&mut input_line).unwrap();
         let inputs = input_line.split_whitespace().collect::<Vec<_>>();
         
         let sample_id = parse_input!(inputs[0], u8);
         let carried_by = CarriedBy::from_integer(parse_input!(inputs[1], i8)).unwrap();
         let rank = SampleRank::from_integer(parse_input!(inputs[2], i8)).unwrap();
         let expertise_gain: Molecules = Molecules::from_letter(inputs[3].chars().next().unwrap());
         let health = SampleHealth::from_integer(parse_input!(inputs[4], i8));
         let cost = Molecules::from_slice(&inputs[5..10]);
         
         let sample = Sample {
            id: sample_id,
            rank,
            health,
            cost,
            expertise_gain
         };
         match carried_by {
            CarriedBy::Me => {
               self.my_robot.held_samples.push(sample);
            },
            CarriedBy::Other => {
               self.enemy_robot.held_samples.push(sample)
            },
            CarriedBy::Cloud => {
               self.cloud.push(sample)
            }
         }
      }
   }

   pub fn process_turn(&mut self) -> Command {
      if self.my_robot.eta > 0 {
         return Command::Wait;
      }
      // TODO: try project strategy
      match self.goal {
         GameGoals::TakeSamples => {
            // TODO: take perspective samples from cloud
            return self.take_samples();
         },
         GameGoals::ResearchSamples => {
            // TODO: try strategy of getting at least two samples with bigger health values
            // TODO: drop samples that cannot be produced to the cloud
            // TODO: take more samples if I have less than 2 good ones
            return self.research_samples();
         },
         GameGoals::GatherMolecules => {
            return self.gather_molecules();
         }
         GameGoals::ProduceMedicine => {
            return self.produce_medicine();
         },
         GameGoals::DropSamples => {
            return self.drop_samples();
         }
      }
   }

   fn take_samples(&mut self) -> Command {
      if self.my_robot.has_maximum_samples() {
         self.goal = GameGoals::ResearchSamples;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Sample {
         return Command::Goto(Module::Sample)
      }
      let rank: SampleRank = self.my_robot.pick_sample_based_on_expertise();
      return Command::Connect(ConnectOptions::SampleRank(rank));
   }

   fn research_samples(&mut self) -> Command {
      let sample: &Sample;
      if let Some(found_sample) = self.my_robot.get_unresearched_sample() {
         sample = found_sample;
      } else {
         self.goal = GameGoals::GatherMolecules;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Diagnosis {
         return Command::Goto(Module::Diagnosis);
      }
      return Command::Connect(ConnectOptions::SampleId(sample.id));
   }

   fn gather_molecules(&mut self) -> Command {
      if !self.my_robot.can_produce_held_samples(&self.available) {
         if self.my_robot.held_samples.len() == Robot::MAX_SAMPLES {
            self.goal = GameGoals::DropSamples;
         } else {
            self.goal = GameGoals::TakeSamples;
         }
         return self.process_turn();
      }
      if self.my_robot.has_maximum_molecules() || self.my_robot.has_enough_molecules() {
         self.goal = GameGoals::ProduceMedicine;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Molecule {
         return Command::Goto(Module::Molecule);
      }
      if let Some(next_molecule) = self.my_robot.pick_best_molecule(&self.available) {
            return Command::Connect(ConnectOptions::MoleculeType(next_molecule));
      } else {
         self.goal = GameGoals::ProduceMedicine;
         return self.process_turn();
      }
   }

   fn produce_medicine(&mut self) -> Command {
      let sample: &Sample;
      if let Some(found_sample) = self.my_robot.get_most_interesting_ready_sample() {
         sample = found_sample;
      } else if self.my_robot.held_samples.len() > 0 {
         self.goal = GameGoals::GatherMolecules;
         return self.process_turn();
      } else {
         self.goal = GameGoals::TakeSamples;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Laboratory {
         return Command::Goto(Module::Laboratory);
      }

      return Command::Connect(ConnectOptions::SampleId(sample.id));
   }
   fn drop_samples(&mut self) -> Command {
      if self.my_robot.held_samples.len() == 0 {
         self.goal = GameGoals::TakeSamples;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Diagnosis {
         return Command::Goto(Module::Diagnosis);
      }

      return Command::Connect(ConnectOptions::SampleId(self.my_robot.held_samples[0].id));
   }
}

impl Add for Molecules {
   type Output = Molecules;

   fn add(mut self, other: Molecules) -> Molecules {
      self.a += other.a;
      self.b += other.b;
      self.c += other.c;
      self.d += other.d;
      self.e += other.e;
      self
   }
}

impl Sub for Molecules {
   type Output = Molecules;

   fn sub(mut self, other: Molecules) -> Molecules {
      self.a -= other.a;
      self.b -= other.b;
      self.c -= other.c;
      self.d -= other.d;
      self.e -= other.e;
      self
   }
}

impl Module {
   const SAMPLE: &str = "SAMPLES";
   const DIAGNOSIS: &str = "DIAGNOSIS";
   const MOLECULES: &str = "MOLECULES";
   const LABORATORY: &str = "LABORATORY";
   const SPAWN: &str = "START_POS";

   fn from_str(s: &str) -> Result<Self, &'static str> {
      match s {
         Self::SAMPLE => Ok(Module::Sample),
         Self::DIAGNOSIS => Ok(Module::Diagnosis),
         Self::MOLECULES => Ok(Module::Molecule),
         Self::LABORATORY => Ok(Module::Laboratory),
         Self::SPAWN => Ok(Module::Spawn),
         _ => Err("Invalid module name"),
      }
   }

   fn as_str(&self) -> &str {
      match self {
         Module::Spawn => Self::SPAWN,
         Module::Sample => Self::SAMPLE,
         Module::Diagnosis => Self::DIAGNOSIS,
         Module::Molecule => Self::MOLECULES,
         Module::Laboratory => Self::LABORATORY,
      }
   }
}

fn main() {
   let mut state_machine = Memory::new();
   state_machine.parse_initial_input();
   loop {
      state_machine.parse_turn_input();
      // eprintln!("{:?}", state_machine);
      println!("{}", state_machine.process_turn().to_string());
   }
}
