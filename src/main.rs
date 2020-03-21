use std::thread;
use std::time::Duration;

fn generate_workout(intensity:u32, random_number:u32) {
  let mut expensive_result = Catcher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_result.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}

struct Catcher<T> where T : Fn(u32) -> u32 {
  calculation: T,
  value: Option<u32>
}

impl<T> Catcher<T> where T : Fn(u32) -> u32 {
  pub fn new(calculation : T) -> Self {
    Catcher {
      calculation,
      value: None
    }
  }

  // Incomplete implementation
  pub fn value(& mut self, arg : u32) -> u32 {
    match self.value {
      Some(a) => a,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;
  generate_workout(
      simulated_user_specified_value,
      simulated_random_number
  );
}