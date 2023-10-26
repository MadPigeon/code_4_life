use super::command;
use super::connect_options;
use super::input_reading;
use super::module;
use super::molecules;
use super::robot;
use super::sample;

#[derive(Debug)]
enum GameGoals {
   TakeSamples,
   ResearchSamples,
   GatherMolecules,
   ProduceMedicine,
   DropSamples,
}

#[derive(Debug)]
pub struct Memory {
   goal: GameGoals,
   my_robot: robot::Robot,
   enemy_robot: robot::Robot,
   available: molecules::Molecules,
   cloud: Vec<sample::Sample>,
}

impl Memory {
   pub fn new() -> Self {
      Self {
         goal: GameGoals::TakeSamples,
         my_robot: robot::Robot::new(),
         enemy_robot: robot::Robot::new(),
         available: molecules::Molecules::new(),
         cloud: Vec::new(),
      }
   }

   pub fn ignore_initial_input(&self) {
      input_reading::ignore_projects();
   }

   pub fn parse_turn_input(&mut self) {
      let (my_robot, enemy_robot, cloud, available) = input_reading::parse_turn_input();
      self.my_robot = my_robot;
      self.enemy_robot = enemy_robot;
      self.cloud = cloud;
      self.available = available;
   }

   pub fn process_turn(&mut self) -> command::Command {
      if self.my_robot.get_eta() > 0 {
         return command::Command::Wait;
      }
      match self.goal {
         GameGoals::TakeSamples => {
            return self.take_samples();
         }
         GameGoals::ResearchSamples => {
            // TODO: try strategy of getting at least two samples with bigger health values
            return self.research_samples();
         }
         GameGoals::GatherMolecules => {
            return self.gather_molecules();
         }
         GameGoals::ProduceMedicine => {
            return self.produce_medicine();
         }
         GameGoals::DropSamples => {
            return self.drop_samples();
         }
      }
   }

   fn take_samples(&mut self) -> command::Command {
      // TODO: take perspective samples from cloud
      if self.my_robot.has_maximum_samples() {
         self.goal = GameGoals::ResearchSamples;
         return self.process_turn();
      }
      if self.my_robot.get_location() != &module::Module::Sample {
         return command::Command::Goto(module::Module::Sample);
      }
      let best_sample = self.my_robot.pick_sample_based_on_expertise();
      return command::Command::Connect(connect_options::ConnectOptions::SampleRank(best_sample));
   }

   fn research_samples(&mut self) -> command::Command {
      let sample: &sample::Sample;
      if let Some(found_sample) = self.my_robot.get_unresearched_sample() {
         sample = found_sample;
      } else {
         self.goal = GameGoals::DropSamples;
         return self.process_turn();
      }
      if self.my_robot.get_location() != &module::Module::Diagnosis {
         return command::Command::Goto(module::Module::Diagnosis);
      }
      return command::Command::Connect(connect_options::ConnectOptions::SampleId(
         sample.get_id(),
      ));
   }

   fn gather_molecules(&mut self) -> command::Command {
      if !self.my_robot.can_produce_one_held_sample(&self.available) {
         if self.my_robot.has_maximum_samples() {
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
      if self.my_robot.get_location() != &module::Module::Molecule {
         return command::Command::Goto(module::Module::Molecule);
      }
      if let Some(next_molecule) = self.my_robot.pick_best_molecule(&self.available) {
         return command::Command::Connect(connect_options::ConnectOptions::MoleculeType(next_molecule));
      } else {
         self.goal = GameGoals::ProduceMedicine;
         return self.process_turn();
      }
   }

   fn produce_medicine(&mut self) -> command::Command {
      let sample: &sample::Sample;
      if let Some(found_sample) = self.my_robot.get_most_interesting_ready_sample() {
         sample = found_sample;
      } else if self.my_robot.get_held_samples().len() > 0 {
         self.goal = GameGoals::GatherMolecules;
         return self.process_turn();
      } else {
         self.goal = GameGoals::TakeSamples;
         return self.process_turn();
      }
      if self.my_robot.get_location() != &module::Module::Laboratory {
         return command::Command::Goto(module::Module::Laboratory);
      }

      return command::Command::Connect(connect_options::ConnectOptions::SampleId(
         sample.get_id(),
      ));
   }

   fn drop_samples(&mut self) -> command::Command {
      let samples_to_drop: Vec<&sample::Sample> =
         self.my_robot.get_impossible_samples(&self.available);
      if samples_to_drop.len() == 0 {
         if self.my_robot.has_enough_samples() {
            self.goal = GameGoals::GatherMolecules;
         } else {
            self.goal = GameGoals::TakeSamples;
         }
         return self.process_turn();
      }
      if self.my_robot.get_location() != &module::Module::Diagnosis {
         return command::Command::Goto(module::Module::Diagnosis);
      }

      return command::Command::Connect(connect_options::ConnectOptions::SampleId(
         samples_to_drop[0].get_id(),
      ));
   }
}
