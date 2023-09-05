/**
 * Bring data on patient samples from the diagnosis machine to the laboratory
 * with enough molecules to produce medicine!
 **/

/**
 * 1. Collect sample data at the DIAGNOSIS module
 * 2. Gather required molecules for the medicines at the MOLECULES module
 * 3. Produce the medicines at the LABORATORY modue
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
enum Module {
   Sample,
   Diagnosis,
   Molecule,
   Laboratory,
   Spawn,
}

#[derive(Debug)]
enum CarriedBy {
   Me = 0,
   Other = 1,
   Cloud = -1,
}

#[derive(Debug)]
enum RoboState {
   SampleModule,
   DiagnosisModule,
   MoleculeModule,
   LaboratoryModule,
   Spawn,
   Idle,
   Moving,
   CompletingProject,
}

#[derive(Debug)]
enum SampleRank {
   LotsOfHealth = 3,
   SomeHealth = 2,
   LittleHealth = 1,
}

#[derive(Debug)]
enum ConnectOptions {
   SampleId,
   SampleRank(SampleRank),
   MoleculeType,
}

#[derive(Debug)]
enum Commands {
   Goto(Module),
   Connect(ConnectOptions),
   Wait,
}

const SAMPLE_INVENTORY_SPACE: u8 = 3;
const MOLECULE_INVENTORY_SPACE: u8 = 10;

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
   carried_by: CarriedBy,
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
   projects: Vec<Molecules>,
   my_robot: Robot,
   enemy_robot: Robot,
   available: Molecules,
   samples: Vec<Sample>,
}

#[derive(Debug)]
struct Robot {
   target: Module,
   eta: u8,
   score: i16,
   storage: Molecules,
   expertise: Molecules,
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

   pub fn count(&self) -> i8 {
      self.a + self.b + self.c + self.d + self.e
   }

   pub fn list_missing(&self) -> String {
      let mut missing = String::new();
      if self.a < 0 {
         missing += &"A".repeat(-self.a as usize);
      }
      if self.b < 0 {
         missing += &"B".repeat(-self.b as usize);
      }
      if self.c < 0 {
         missing += &"C".repeat(-self.c as usize);
      }
      if self.d < 0 {
         missing += &"D".repeat(-self.d as usize);
      }
      if self.e < 0 {
         missing += &"E".repeat(-self.e as usize);
      }
      missing
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
}

impl Robot {
   pub fn new() -> Self {
      Self {
         target: Module::Spawn,
         eta: 0,
         score: 0,
         storage: Molecules::new(),
         expertise: Molecules::new(),
      }
   }

   fn set_from_inputs(&mut self, inputs: Vec<&str>) {
      self.target = Module::from_str(inputs[0].trim()).unwrap();
      self.eta = parse_input!(inputs[1], u8);
      self.score = parse_input!(inputs[2], i16);
      self.storage = Molecules::from_slice(&inputs[3..8]);
      self.expertise = Molecules::from_slice(&inputs[8..13]);
   }
}

impl Sample {
   pub fn new() -> Self {
      Self {
         id: 0,
         carried_by: CarriedBy::Other,
         rank: SampleRank::LittleHealth,
         health: SampleHealth::Unresearched,
         cost: Molecules::new(),
         expertise_gain: Molecules::new()
      }
   }
}

impl Memory {
   pub fn new() -> Self {
      Self {
         projects: Vec::new(),
         my_robot: Robot::new(),
         enemy_robot: Robot::new(),
         available: Molecules::new(),
         samples: Vec::new(),
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

   pub fn process_turn_input(&mut self) {
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

      input_line.clear();
      io::stdin().read_line(&mut input_line).unwrap();
      let sample_count = parse_input!(input_line, u16);
      let mut samples: Vec<Sample> = Vec::new();
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
            carried_by,
            rank,
            health,
            cost,
            expertise_gain
         };
         eprintln!("{:?}", sample);
         samples.push(sample);
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
      state_machine.process_turn_input();
      eprintln!("{:?}", state_machine);
      println!("{}", "WAIT");
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
      assert_eq!(-5, negative.count());
   }

   #[test]
   fn new_is_empty() {
      let empty = Molecules::new();
      assert_eq!(0, empty.count());
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
      assert_eq!(15, a.count());
      assert!(!a.is_not_positive());
      assert_eq!(25, c.count());
   }

   #[test]
   fn lists_missing() {
      let negative = Molecules {
         a: 3,
         b: -1,
         c: -3,
         d: -1,
         e: -1,
      };
      assert_eq!(-3, negative.count());
      assert_eq!("BCCCDE", negative.list_missing());
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
