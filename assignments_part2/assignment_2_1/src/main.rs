fn concat_strings(s1: &String, s2: &String) -> String {
    let new_string = s1.to_string()+s2;
    new_string
}


fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);
}