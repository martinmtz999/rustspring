// fn main() {
//     let operation = |a: i32, b: i32| {
//         a * b
//     };

//     println!("Result: {}", operation(10, 5));
// }

// fn track_changes() {
//     let mut tracker = 0;
//     let mut update = || {
//         tracker += 1;
//         println!("Current Value of Tracker: {}", tracker)
//     };

//     update();
//     update();
// }

// fn main() {
//     track_changes();
// }




// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     vec.into_iter().map(f).collect()
// }

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x)); // Apply the closure
//     }
//     result
// }

// fn main() {
//     let numbers = vec![1, 2, 3];

//     let doubled = process_vector(numbers.clone(), |x| {
//         // Implement: multiply each number by 2
//         x * 2
//     });

//     let replaced = process_vector(numbers, |x| {
//         // Implement: if number > 2, replace with 0, else keep number
//         if x > 2{
//             x * 0       
//         }
//         else{
//             x
//         }
//     });

//     println!("Doubled: {:?}", doubled);
//     println!("Replaced: {:?}", replaced);
// }

use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache{
            computation,
            result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.result{
            Some(value) => {
                value.clone()
            }
            None => {
                thread::sleep(Duration::from_secs(2));
                let value = (self.computation)();
                self.result = Some(value.clone());
                value
            }
        }
        
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

