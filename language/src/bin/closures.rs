use std::thread;
use std::time::Duration;

/*
Closures can capture values from their environment in three ways, which directly map to the three
ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably.
These are encoded in the three Fn traits as follows:

    * FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s
    environment. To consume the captured variables, the closure must take ownership of these
    variables and move them into the closure when it is defined. The Once part of the name
    represents the fact that the closure can’t take ownership of the same variables more than once,
    so it can be called only once.
    * FnMut can change the environment because it mutably borrows values.
    * Fn borrows values from the environment immutably.

If you want to force the closure to take ownership of the values it uses in the environment, you
can use the move keyword before the parameter list. This technique is mostly useful when passing
a closure to a new thread to move the data so it’s owned by the new thread.
    e.g.  let equal_to_x = move |z| z == x;
*/

/// Cacher implements the memoization pattern
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
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



fn main() {
    let simulated_user_specified_value = 10;
    let mut rnum = 0;

    let is_odd = |n: &i32| {
        rnum += 1;
        n % 2 != 0
    };

    let odds = (0..=50).filter( is_odd ).collect::<Vec<i32>>();
    for n in odds { println!("{}", n) };
    println!("rnum {}", rnum);
//    generate_workout(
//        simulated_user_specified_value,
//        simulated_random_number
//    );
}