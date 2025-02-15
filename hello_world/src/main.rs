fn string_copy (my_string: String) {
        let borrow_word = &my_string;
        println!("{} == {}", my_string, borrow_word);
}


fn main () {
    let word = "VGRTU".to_string();
    string_copy(word);
}