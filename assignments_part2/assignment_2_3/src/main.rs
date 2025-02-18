#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    let mut counter = 0;
    for i in low..=high{
        *total += i;
    }
    println!("{}", *total);
    
}

fn main() {
    let  low = 0;
    let  high = 100;
    let mut total = 0;
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050

    sum(&mut total, low, high);

}