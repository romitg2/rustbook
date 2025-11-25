use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 10;
    let random_number = 7;

    generate_workout(intensity, random_number);

    println!("Hello, world!");
}

fn expensive_calculation(intensity: u32) -> u32 {
    println!("Starting expensive calculation...");
    thread::sleep(Duration::from_secs(2));
    println!("Expensive calculation done.");
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,

}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if v == arg {
                    v
                } else {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            },
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_closure = Cacher::new(|intensity: u32| {
        println!("Starting expensive calculation...");
        thread::sleep(Duration::from_secs(2));
        println!("Expensive calculation done.");
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}