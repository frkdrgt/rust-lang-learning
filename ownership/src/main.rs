fn main() {
    let text = String::from("Hello, World");
    let size = get_length(&text);
    println!("{}", size);
    
    // Mutable Örnek
    let mut second_text = String::from("Hello, Second Text");
    let new_text = add_string(&mut second_text);
    println!("{}",new_text);
}
fn get_length(string : &String) -> usize {
    string.len()
}

fn add_string(string : &mut String) -> &mut String {
     string.push_str(", Hello Rusty");
     return string
}