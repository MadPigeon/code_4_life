use super::carried_by;
use super::molecules;
use super::robot;
use super::sample;
use std::io;

pub fn ignore_projects() -> () {
   let mut input_line = String::new();
   io::stdin().read_line(&mut input_line).unwrap();
   let project_count = parse_input!(input_line, u8);
   for _ in 0..project_count {
      io::stdin().read_line(&mut input_line).unwrap();
   }
}

pub fn parse_turn_input() -> (
   robot::Robot,
   robot::Robot,
   Vec<sample::Sample>,
   molecules::Molecules,
) {
   let mut input_line = String::new();
   io::stdin().read_line(&mut input_line).unwrap();
   let mut my_robot =
      robot::Robot::new_from_inputs(input_line.split_whitespace().collect::<Vec<_>>());

   input_line.clear();
   io::stdin().read_line(&mut input_line).unwrap();
   let mut enemy_robot =
      robot::Robot::new_from_inputs(input_line.split_whitespace().collect::<Vec<_>>());

   input_line.clear();
   io::stdin().read_line(&mut input_line).unwrap();
   let inputs = input_line.split_whitespace().collect::<Vec<_>>();
   let available = molecules::Molecules::from_slice(&inputs[0..5]);

   let mut cloud = Vec::new();

   input_line.clear();
   io::stdin().read_line(&mut input_line).unwrap();
   let sample_count = parse_input!(input_line, u16);
   for _ in 0..sample_count {
      input_line.clear();
      io::stdin().read_line(&mut input_line).unwrap();
      let inputs = input_line.split_whitespace().collect::<Vec<_>>();

      let sample_id = parse_input!(inputs[0], u8);
      let carried_by = carried_by::CarriedBy::from_integer(parse_input!(inputs[1], i8)).unwrap();
      let rank = sample::SampleRank::from_integer(parse_input!(inputs[2], i8)).unwrap();
      let expertise_gain: molecules::Molecules =
         molecules::Molecules::from_letter(inputs[3].chars().next().unwrap());
      let health = sample::SampleHealth::from_integer(parse_input!(inputs[4], i8));
      let cost = molecules::Molecules::from_slice(&inputs[5..10]);

      let sample = sample::Sample::new(sample_id, rank, health, cost, expertise_gain);
      match carried_by {
         carried_by::CarriedBy::Me => {
               my_robot.append_sample(sample);
         }
         carried_by::CarriedBy::Other => enemy_robot.append_sample(sample),
         carried_by::CarriedBy::Cloud => cloud.push(sample),
      }
   }
   return (my_robot, enemy_robot, cloud, available);
}
