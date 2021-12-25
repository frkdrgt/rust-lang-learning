# Control Flow
Rust dilinde belli koşulları yerine getirmek için **if** ifadeleri ve **döngüler** kullanılmaktadır.
## if Koşulu
`if` koşulu verilen koşul sağlandığı zaman içerisinde kod bloğunu çalıştırmaktır. Eğer koşul uygun değil ise if içerisini çalıştırmamaktadır. 
Birden fazla koşul vermemiz gerektiğinde `if` koşuluna ek olarak ``else if`` ve ``else`` koşullarını dahil edebiliriz.
```sh
fn main() {
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
}
```
## let Deyiminde if Kullanımı
Rust dilinde `if` bir **expression** olduğu için let deyiminin sağ tarafında kullanabiliriz.
```sh
fn main() {
    let condition = true;
    let result = if condition { true } else { false };
    println!("{}",result);
}
```
```sh
fn main() {
    let small_number = 2;
    let result = if small_number > 1 { "Sayı büyük" } else { "Sayı küçük "};
    println!("{}", result);
}
```
# Loops
Rust dilinde 3 çeşit döngü bulunmaktadır.
`loop`,`while`,`for`
## loop Döngüsü
`loop` döngüsü program içerisinde sürekli olarak çalışan bir döngü türüdür. Programı durduruna dek yada kendimiz müdahale etmediğimiz sürece çalışacaktır.
```sh
fn main() {
    loop {
        println!("Hello, world!");
    }
}
```
`loop` döngüsünden çıkmanın bir yolu ise **break** anahtar kelimesini kullanmamızdır.
```sh
fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Hello, world!");
        if counter == 10 {
            break;
        }
    }
}
```
`loop` döngüsünde **break** ile değer döndürebiliyoruz.
let deyiminin sağ tarafında loop döngüsü başlatıp istediğimiz koşul sağlandığı zaman **break** sözdiziminden sonra döndürmek istediğimiz değişkeni yada değeri yazarak **loop_result** değişkenine aktarmış oluyoruz.
```sh
fn main() {
    let mut loop_counter = 0;
    let loop_result = loop {
        loop_counter += 1;
        println!("Hello, world!");
        if loop_counter == 10 {
            break "Loop döngüsü bitti";
        }
    };
    println!("{}", loop_result);
}
```
## while Döngüsü
while döngüsü içerisine verilen koşul doğru ise çalışır değilse döngü çalışmamaktadır.
```sh
fn main() {
    let mut gift_count = 3;
    while gift_count != 0 {
        println!("{}!", gift_count);

        gift_count -= 1;
    }
    println!("gift_count değeri 0 oldu ve while döngüsünden çıkıldı");
}
```
## for Döngüsü
`for` döngülerinin güvenliği ve kısa olması, onları Rust'ta en sık kullanılan döngü yapısı haline getirir.
for döngüsü ile diziler içerisindeki verilere erişebilir yada belli aralıklarda sayılar vb. veriler oluşturabiliriz.
```sh
fn main() {
    let number_array = [5,4,3,2,1];
    for number in number_array {
        println!("dizi elemanı {}", number);
    }
}
```
```sh
fn main() {
    for counting_number in 1..9 {
        println!("counting {}", counting_number);
    }
}
```