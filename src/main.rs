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
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ty) => ($x.trim().parse::<$t>().unwrap());
}

enum Module {
    Sample,
    Diagnosis,
    Molecule,
    Laboratory,
}

enum CarriedBy {
    Me = 0,
    Other = 1,
    Cloud = -1,
}

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

enum SampleRank {
    LotsOfHealth = 3,
    SomeHealth = 2,
    LittleHealth = 1,
}

enum ConnectOptions {
    SampleId,
    SampleRank(SampleRank),
    MoleculeType,
}

enum Commands {
    Goto(Module),
    Connect(ConnectOptions),
    Wait,
}

const SAMPLE_INVENTORY_SPACE: u8 = 3;
const MOLECULE_INVENTORY_SPACE: u8 = 10;
const ILLEGAL_VALUE: i8 = -1;

#[derive(Clone, Debug)]
struct Molecules {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
}

struct Samples {
    id: u8,
    carried_by: CarriedBy,
    rank: SampleRank,
    health: u8,
    cost: Molecules
}

struct TurnInput {
    target: String,
    eta: u8,
    score: i32,
    storage: Molecules,
    expertise: Molecules,
    available: Molecules,
    samples: Samples
}

impl TurnInput {
    /* pub fn new() -> Self {
        
    } */
}

struct Robot {
    latest_input: TurnInput,
    projects: Vec<Molecules>
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

	pub fn count(&self) -> i32 {
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

fn parse_initial_input() -> Vec<Molecules> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let project_count = parse_input!(input_line, i32);
    
    let mut projects = Vec::new();
    for _ in 0..project_count {
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split_whitespace().collect::<Vec<_>>();
        let a = parse_input!(inputs[0], i32);
        let b = parse_input!(inputs[1], i32);
        let c = parse_input!(inputs[2], i32);
        let d = parse_input!(inputs[3], i32);
        let e = parse_input!(inputs[4], i32);
        projects.push(Molecules {a,b,c,d,e});
    }

    projects
}

fn main() {
    let projects = parse_initial_input();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Molecules;

    #[test]
    fn sets_negative() {
        let negative = Molecules{a:-1,b:-1,c:-1,d:-1,e:-1};
        assert_eq!(-5, negative.count());
    }

    #[test]
    fn new_is_empty() {
        let empty = Molecules::new();
        assert_eq!(0, empty.count());
    }

    #[test]
    fn addition_works() {
        let a = Molecules{a:1,b:2,c:3,d:4,e:5};
        let b = Molecules{a:4,b:3,c:2,d:1,e:0};
        let c = a.clone() + b;
        assert_eq!(15, a.count());
        assert!(!a.is_not_positive());
        assert_eq!(25, c.count());
    }

    #[test]
    fn lists_missing() {
        let negative = Molecules{a:3,b:-1,c:-3,d:-1,e:-1};
        assert_eq!(-3, negative.count());
        assert_eq!("BCCCDE", negative.list_missing());
    }

    #[test]
    fn not_positive() {
        let semi_positive = Molecules{a:1,b:2,c:-6,d:0,e:0};
        let non_positive = Molecules{a:0,b:0,c:-2,d:0,e:0};
        assert!(!semi_positive.is_not_positive());
        assert!(non_positive.is_not_positive());
    }
}