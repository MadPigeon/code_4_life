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
use std::fmt;

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
   ProduceMedicine
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
   A,B,C,D,E, None
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
   storage: Molecules,
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
         Molecule::None => '~',
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
}

impl fmt::Display for Module {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Module::Spawn => write!(f, "START_POS"),
         Module::Sample => write!(f, "SAMPLES"),
         Module::Diagnosis => write!(f, "DIAGNOSIS"),
         Module::Molecule => write!(f, "MOLECULES"),
         Module::Laboratory => write!(f, "LABORATORY"),
      }
   }
}

impl fmt::Display for ConnectOptions {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         ConnectOptions::SampleRank(rank) => write!(f, "{}", rank.as_value()),
         ConnectOptions::MoleculeType(molecule_type) => write!(f, "{}", molecule_type.as_char()),
         ConnectOptions::SampleId(id) => write!(f, "{}", id),
      }
   }
}

impl fmt::Display for Command {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Command::Goto(module) => write!(f, "GOTO {}", module),
         Command::Connect(options) => write!(f, "CONNECT {}", options),
         _ => write!(f, "WAIT"),
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

impl SampleRank {
   fn from_integer(value: i8) -> Option<Self> {
      match value {
         3 => Some(SampleRank::LotsOfHealth),
         2 => Some(SampleRank::SomeHealth),
         1 => Some(SampleRank::LittleHealth),
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
      if slice.len() >= 5 {
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

   // TODO: make return Optional<Molecule>
   fn get_next_molecule(&self) -> Molecule {
      if self.a > 0 {
         return Molecule::A;
      }
      if self.b > 0 {
         return Molecule::B
      }
      if self.c > 0 {
         return Molecule::C;
      }
      if self.d > 0 {
         return Molecule::D;
      }
      if self.e > 0 {
         return Molecule::E;
      }
      panic!("no missing molecules");
   }
}

impl Robot {
   pub fn new() -> Self {
      Self {
         location: Module::Spawn,
         eta: 0,
         score: 0,
         storage: Molecules::new(),
         expertise: Molecules::new(),
         held_samples: Vec::new()
      }
   }

   const SAMPLE_INVENTORY_SPACE: usize = 3;
   const MOLECULE_INVENTORY_SPACE: i8 = 10;
   fn set_from_inputs(&mut self, inputs: Vec<&str>) {
      self.location = Module::from_str(inputs[0].trim()).unwrap();
      self.eta = parse_input!(inputs[1], u8);
      self.score = parse_input!(inputs[2], i16);
      self.storage = Molecules::from_slice(&inputs[3..8]);
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

   fn has_maximum_samples(&self) -> bool {
      return self.held_samples.len() >= Self::SAMPLE_INVENTORY_SPACE
   }

   fn has_maximum_molecules(&self) -> bool {
      return self.storage.len() == Self::MOLECULE_INVENTORY_SPACE;
   }

   // TODO: make return Option<Molecule> and remove Molecule::None
   fn pick_best_molecule(&self, available: &Molecules) -> Molecule {
      let mut sorted_samples: Vec<&Sample> = self.held_samples.iter()
         .map(|sample| sample).collect();
      sorted_samples.sort_by_key(|&sample| match sample.health {
         SampleHealth::Researched(health) => -(health as i8),
         _ => 0i8
      });
      // go through every sample
      let mut held_molecules = self.storage.clone();
      let mut additional_expertise = Molecules::new();
      for sample in sorted_samples {
         eprintln!("analyzing sample: {:?}", sample);
         let needed_molecules = sample.cost.clone() - self.expertise.clone() - held_molecules.clone();
         // check if already have everything needed
         if needed_molecules.is_not_positive() {
            eprintln!("have everything needed for it");
            held_molecules = held_molecules - (sample.cost.clone() - self.expertise.clone()).set_minues_to_zero();
            additional_expertise = additional_expertise + sample.expertise_gain.clone();
            continue;
         }
         if !available.has_enough(&needed_molecules.set_minues_to_zero()) {
            eprintln!("not enough available");
            continue;
         }
         if needed_molecules.set_minues_to_zero().len() > (Self::MOLECULE_INVENTORY_SPACE - self.storage.len()) {
            eprintln!("not enough inventory space for: {:?}", needed_molecules);
            continue;
         }
         eprintln!("picking from needed: {:?}", needed_molecules);
         return needed_molecules.get_next_molecule();
      }
      return Molecule::None;
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
      match self.goal {
         GameGoals::TakeSamples => {
            return self.take_samples();
         },
         GameGoals::ResearchSamples => {
            return self.research_samples();
         },
         GameGoals::GatherMolecules => {
            return self.gather_molecules();
         }
         GameGoals::ProduceMedicine => {
            // TODO: write logic for producing medicine
            return Command::Goto(Module::Laboratory);
         }
         // TODO: add goal for dropping samples that cannot be done
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
      return Command::Connect(ConnectOptions::SampleRank(SampleRank::LittleHealth));
   }

   fn research_samples(&mut self) -> Command {
      let sample: &Sample;
      match self.my_robot.get_unresearched_sample() {
         None => {
            self.goal = GameGoals::GatherMolecules;
            return self.process_turn();
         }
         Some(found_sample) => {
            sample = found_sample;
         }
      }
      if self.my_robot.location != Module::Diagnosis {
         return Command::Goto(Module::Diagnosis);
      }
      return Command::Connect(ConnectOptions::SampleId(sample.id));
   }

   fn gather_molecules(&mut self) -> Command {
      // TODO: safeguard against full molecules with no finisheable projects
      if self.my_robot.has_maximum_molecules() {
         self.goal = GameGoals::ProduceMedicine;
         return self.process_turn();
      }
      if self.my_robot.location != Module::Molecule {
         return Command::Goto(Module::Molecule);
      }
      let next_molecule = self.my_robot.pick_best_molecule(&self.available);
      match next_molecule {
         Molecule::None => {
            self.goal = GameGoals::ProduceMedicine;
            return self.process_turn();
         },
         _ => {
            return Command::Connect(ConnectOptions::MoleculeType(next_molecule))
         }
      }
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
   fn from_str(s: &str) -> Result<Self, &'static str> {
      match s {
         "SAMPLES" => Ok(Module::Sample),
         "DIAGNOSIS" => Ok(Module::Diagnosis),
         "MOLECULES" => Ok(Module::Molecule),
         "LABORATORY" => Ok(Module::Laboratory),
         "START_POS" => Ok(Module::Spawn),
         _ => Err("Invalid module name"),
      }
   }
}

fn main() {
   let mut state_machine = Memory::new();
   state_machine.parse_initial_input();
   loop {
      state_machine.parse_turn_input();
      // eprintln!("{:?}", state_machine);
      println!("{}", state_machine.process_turn());
   }
}

#[cfg(test)]
mod tests {
   use crate::Molecules;

   #[test]
   fn sets_negative() {
      let negative = Molecules {
         a: -1,
         b: -1,
         c: -1,
         d: -1,
         e: -1,
      };
      assert_eq!(-5, negative.len());
   }

   #[test]
   fn new_is_empty() {
      let empty = Molecules::new();
      assert_eq!(0, empty.len());
   }

   #[test]
   fn addition_works() {
      let a = Molecules {
         a: 1,
         b: 2,
         c: 3,
         d: 4,
         e: 5,
      };
      let b = Molecules {
         a: 4,
         b: 3,
         c: 2,
         d: 1,
         e: 0,
      };
      let c = a.clone() + b;
      assert_eq!(15, a.len());
      assert!(!a.is_not_positive());
      assert_eq!(25, c.len());
   }

   #[test]
   fn not_positive() {
      let semi_positive = Molecules {
         a: 1,
         b: 2,
         c: -6,
         d: 0,
         e: 0,
      };
      let non_positive = Molecules {
         a: 0,
         b: 0,
         c: -2,
         d: 0,
         e: 0,
      };
      assert!(!semi_positive.is_not_positive());
      assert!(non_positive.is_not_positive());
   }
}
