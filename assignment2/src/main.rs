fn is_even(n: i32) -> bool{
    n % 2 == 0
}


fn main() {
    let num_arr: [i32; 15] = [15,54,10,9,2,4,9,91,23,16,23,43,84,43,39];

    for i in num_arr{

        let result = if is_even(i) { "Even" } else { "Odd" };
        println!("The number: {} is: {}\n", i, result);
        
        match i{
            _ if i % 3 == 0 && i % 5 == 0 => println!("FizzBuzz"),
            _ if i % 3 == 0 => println!("Fizz"),
            _ if i % 5 == 0 => println!("Buzz"),
            _ => println!("None"), 
        }
        
    }

    let mut count = 0;
    let mut sum = 0;
    
    while count < num_arr.len(){
        sum += num_arr[count];
        count += 1;
    }
    println!("\n\nTotal of the array: {}",sum);

    let mut largest = num_arr[0];
    let mut counter = 1;

    loop{
        if counter >= num_arr.len(){
            println!("Largest number in the array is: {}", largest);
            break;
        }

        if num_arr[counter] > largest {
            largest = num_arr[counter];
        }
        counter += 1;
   
    }
}    