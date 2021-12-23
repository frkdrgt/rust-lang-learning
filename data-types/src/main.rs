fn main() {
    //Integer Type
    let number : i32 = -10;
    let count : u32 = 10;

    //Floating-Point Type
    let money = 5.5; //f64
    let amount : f32 = 2.5; //f32

    //Boolean Type
    let t = true;
    let f: bool = false;

    //Char Type
    let firstChar = 'A';
    let heart = '🧡';
    println!("{}",heart);
    let secondChar : char = 'B';

    //Tuple
    let tup: (i32, f64, u8) = (-51, 2.4, 3);
    println!("0. indexin değeri {}", tup.0);
    let tup2 = (100,500,3);
    let (x, y, z) = tup2; //pattern matching
    println!("Tuple Y değeri {}",y);

    //Array
    let numbers = [1, 2, 3];
    let months = ["January", "February", "March"];

    let counts : [i32;5] = [1, 2, 3, 4, 5];

    let ones = [1; 10];

    println!("{}",numbers[1]);
}
