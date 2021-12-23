fn third_print() {
    println!("Hello from third print");
}

fn main() {
    third_print();

    println!("Hello, world!");

    second_print();

    print_number(5);

    print_multiple_number(-4,15);

    let given_number = return_given_number(5);
    println!("{}",given_number);
    
    let total_number = plus_one(99);
    println!("{}", total_number);


    //function expression
    let result = {
        let x = 4;
        x + 1
    };
    println!("Result değişken içeriği : {} ", result);
}

fn second_print() {
    println!("Hello from new function");
}

fn print_number(x: i32) {
    println!("Bana bu sayıyı gönderdin : {}", x);
}

fn print_multiple_number(x: i32, y: u32) {
    println!("X : {} ve Y: {}", x , y);
}

fn return_given_number(x: i32) -> i32 {
    x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}