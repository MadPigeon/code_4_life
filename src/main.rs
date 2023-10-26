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

macro_rules! parse_input {
   ($x:expr, $t:ty) => {
      $x.trim().parse::<$t>().unwrap()
   };
}

mod input_reading;
mod module;
mod carried_by;
mod sample;
mod connect_options;
mod command;
mod molecules;
mod robot;
mod memory;


fn main() {
   let mut state_machine = memory::Memory::new();
   state_machine.ignore_initial_input();
   loop {
      state_machine.parse_turn_input();
      // eprintln!("{:?}", state_machine);
      println!("{}", state_machine.process_turn().to_string());
   }
}
