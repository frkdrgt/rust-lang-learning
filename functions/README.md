# Fonksiyonlar
Rust dilinde fonksiyonların kullanımı yaygındır. Fonksiyonları `fn` anahtar sözcüğü ile tanımlıyoruz. 
Fonksiyon tanımlamada isimlendirirken **snake case** yapısını kullanıyoruz. Yani fonksiyon ismini küçük harflerle ve alt çizgi ile ayırarak yazıyoruz.
```sh
fn main() {
    println!("Hello, world!");

    second_print();
}

fn second_print() {
    println!("Hello from new function");
}
```
Fonksiyonları kod içerisinde main fonksiyonun altında veya üstünde tanımlamamızın bir önemi yoktur. Yani;
```sh
fn third_print(){
    println!("Hello from third print");
}

fn main() {
    third_print();

    println!("Hello, world!");

    second_print();
}

fn second_print() {
    println!("Hello from new function");
}
```
### Parametreli Fonksiyonlar
Diğer programlama dillerindeki gibi Rust dilinde de fonksiyonları parametreli tanımlayabiliriz.
Parantezlerin içerisinde **parametre_adi:veri_tipi** şeklinde tanımlanır.
```sh
fn main() {
    println!("Hello, world!");

    print_number(5);
}

fn print_number(x: i32) {
    println!("Bana bu sayıyı gönderdin : {}", x);
}
```
Birden fazla parametreli şekilde de tanımlayabiliriz.
```sh
fn main() {
    println!("Hello, world!");
    
    print_multiple_number(-4, 15);
}

fn print_multiple_number(x: i32, y: u32) {
    println!("X : {} ve Y: {}", x , y);
}
```
### Değer Döndüren Fonksiyonlar
Rust dilinde fonksiyonların dönüş tipleri **->** işaretinden sonra tanımlanır.
Rust, fonksiyon içerisinde ki son işlevdeki ifadenin değerini döndürür.
Birkaç koşula bağlı işlemlerde fonksiyondan çıkmak için `return` anahtar sözcüğü kullanılır.
```sh
fn main() {
    println!("Hello, world!");

    let given_number = return_given_number(5);
    println!("{}",given_number);
}

fn return_given_number(x: i32) -> i32 {
    x
}
```
### Statement ve Expression İçeren Fonksiyonlar
Rust dilinde fonksiyonlar içerisinde **expression** oluşturmak için süslü parantezler kullanılır.
Fonksiyon yada Macro çağırmak bir **expression**dır. 
Aşağıdaki örnekte {} içerisinde hangi satırların sonunda noktali virgül olduğuna dikkat edilmeli. 
Noktalı virgül ile biten satırlar **statement**. Noktalı virgül ile bitmeyen satır **expression**dır.
Fonksiyonlar **expression** kısımlarını geri döndürür.
```sh
fn main() {
    let result = {
        let x = 4;
        x + 1
    };
    println!("Result değişken içeriği : {} ", result);
}
```
