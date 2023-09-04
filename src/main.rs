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

struct Molecules {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
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

    pub fn set(values: (i32, i32, i32, i32, i32)) -> Self {
		Self {
			a: values.0,
			b: values.1,
			c: values.2,
			d: values.3,
			e: values.4
		}
    }

	pub fn count(&self) -> i32 {
        self.a + self.b + self.c + self.d + self.e
    }

	pub fn list_missing(&self) -> String {
        "A".repeat(0-self.a as usize) +
		&"B".repeat(0-self.b as usize) +
		&"C".repeat(0-self.c as usize) +
		&"D".repeat(0-self.d as usize) +
		&"E".repeat(0-self.e as usize)
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Molecules;

    #[test]
    fn sets_negative() {
        let negative = Molecules::set((-1,-1,-1,-1,-1));
        assert_eq!(-5, negative.count());
    }

    #[test]
    fn new_is_empty() {
        let empty = Molecules::new();
        assert_eq!(0, empty.count());
    }

    #[test]
    fn addition_works() {
        let a = Molecules::set((1,2,3,4,5));
        let b = Molecules::set((4,3,2,1,0));
        let c = a + b;
        // assert_eq!(15, a.count());
        // assert_eq!(10, b.count());
        assert_eq!(25, c.count());
    }
}