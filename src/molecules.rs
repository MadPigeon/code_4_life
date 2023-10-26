use std::ops::{Add, Sub};

#[derive(Debug)]
pub enum Molecule {
   A,
   B,
   C,
   D,
   E,
}

#[derive(Clone, Debug)]
pub struct Molecules {
   a: i8,
   b: i8,
   c: i8,
   d: i8,
   e: i8,
}

impl Molecule {
   pub fn as_char(&self) -> char {
      match self {
         Molecule::A => 'A',
         Molecule::B => 'B',
         Molecule::C => 'C',
         Molecule::D => 'D',
         Molecule::E => 'E',
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

   pub fn set_minues_to_zero(&self) -> Molecules {
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

   pub fn has_enough(&self, required: &Molecules) -> bool {
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

   pub fn get_next_molecule(&self) -> Option<Molecule> {
      if self.a > 0 {
         return Some(Molecule::A);
      }
      if self.b > 0 {
         return Some(Molecule::B);
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

   pub fn has_any_negatives(&self) -> bool {
      self.a < 0 || self.b < 0 || self.c < 0 || self.d < 0 || self.e < 0
   }
}

impl Add<&Molecules> for &Molecules {
   type Output = Molecules;

   fn add(self, other: &Molecules) -> Molecules {
      Molecules {
         a: self.a + other.a,
         b: self.b + other.b,
         c: self.c + other.c,
         d: self.d + other.d,
         e: self.e + other.e,
      }
   }
}

impl Sub<&Molecules> for &Molecules {
   type Output = Molecules;

   fn sub(self, other: &Molecules) -> Molecules {
      Molecules {
         a: self.a - other.a,
         b: self.b - other.b,
         c: self.c - other.c,
         d: self.d - other.d,
         e: self.e - other.e,
      }
   }
}
