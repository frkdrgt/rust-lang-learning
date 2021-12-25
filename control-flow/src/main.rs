fn main() {
    //CONTROL FLOWS
    let number = 1;
    if number > 2 {  
        println!("Sayı 2'ten büyük ");
    }
    else if number > 4 {
        println!("Sayı 4'ten büyük ");
    }
    else {
        println!("Sayı 2 ve 4'ten küçük!");
    }

    let condition = true;
    let result = if condition { true } else { false };
    println!("{}",result);

    let small_number = 2;
    let result = if small_number > 1 { "Sayı büyük" } else { "Sayı küçük "};
    println!("{}", result);

    //LOOPS
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Hello, world!");
        if counter == 10 {
            break;
        }
    }


    let mut loop_counter = 0;

    let loop_result = loop {
        loop_counter += 1;
        println!("Hello, world!");
        if loop_counter == 10 {
            break "Loop döngüsü bitti";
        }
    };
    println!("{}", loop_result);

    //WHILE
    let mut gift_count = 3;

    while gift_count != 0 {
        println!("{}!", gift_count);

        gift_count -= 1;
    }

    println!("gift_count değeri 0 oldu ve while döngüsünden çıkıldı");

    //FOR 
    let number_array = [5,4,3,2,1];
    for number in number_array {
        println!("dizi elemanı {}", number);
    }

    for counting_number in 1..9 {
        println!("counting {}",counting_number);
    }
}
