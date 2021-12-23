# Veri Tipleri

### Integer Type
Rust dilinde integer değişkenleri **signed** ve **unsigned** olarak 2 şekilde tanımlanmaktadır.
Signed değişkenler **i**, unsigned değişkenler **u** harfi ile temsil ediliyor.
Signed değişkenler; pozitif ve negatif sayıları desteklemektedir.
Unsigned değişkenler; sadece pozitif sayıları desteklemektedir.
| Length | Signed | Unsigned |
| ------ | ------ | ------ |
| 8-bit | i8 | u8
| 16-bit | i16 | u16
| 32-bit | i32 | u32
| 64-bit | i64 | u64
| 128-bit | i128 | u128
| arch | isize | usize
```sh
fn main() {
    //Integer Type
    let number : i32 = -10;
    let count : u32 = 10;
}
```

### Floating-Point Type
Çoğu programlama dilinde olan float tipidir. Ondalıklı sayılarda kullanılır.
Rust içerisinde **f32** ve **f64** olarak iki tanımlama şekli bulunmaktadır.
Ondalıklı değişkenler varsayılan olarak **f64** tipinde tanımlanır.
```sh
fn main() {
    let money = 5.5; //f64
    let amount : f32 = 2.5; //f32
}
```
### Boolean Type
Diğer programlama dillerindeki gibi standart olarak iki değeri vardır: **true** ve **false**
```sh
fn main() {
    let t = true;
    let f: bool = false;
}
```
### Character Type
Rust dilinde **char** değişkenler dört bayt boyutundadır. 
Yani sadece ASCII kodlarını değil, aksanlı harfleri yada emojilerde **char** olarak tanımlanabilir. 
Tek tırnak ile belirtilir.
```sh
fn main() {
    let firstChar = 'A';
    let heart = '🧡';
    println!("{}",heart);
    let secondChar : char = 'B';
}
```

### Tuple Type
Tuple, çeşitli türlere sahip bir dizi değeri tek bir bileşik türde gruplandırmanın genel bir yoludur. 
Tuple'ların sabit bir uzunluğu vardır: bir kez bildirildiğinde, boyut olarak büyüyemez veya küçülemezler.
Aşağıdaki gibi farklı değişken tiplerinde veriler tanımlayabiliriz.
```sh
fn main(){
    let tup: (i32, f64, u8) = (-51, 2.4, 3);
}
```
Bir başka yöntem olarak **pattern matching** kullanabiliriz.
Pattern matching ile eşlediğimiz zaman **Y** değeri **2.4** olarak ekrana çıktı verecektir.
```sh
fn main(){
    let tup = (-51, 2.4, 3);
    let (x, y, z) = tup;
    println!("Tuple Y değeri {}",y);
}
```
Pattern matching yöntemine ek olarak **nokta(.)** ile değerlerin indexini kullanabiliriz.
```sh
fn main(){
    let tup: (i32, f64, u8) = (-51, 2.4, 3);
    println!("0. indexin değeri {}", tup.0);
}
```
### Array Type
Diziler, tupledan farklı olarak sadece aynı türdeki verilerden oluşmaktadır.
Rust dilinde diziler sabit bir uzunluğa sahiptir.
Köşeli parantezler içerisine virgül ile ayırarak tanımlamalar yapılır.
```sh
fn main(){
    let numbers = [1, 2, 3];
    let months = ["January", "February", "March"];
}
```
Rust dilinde dizi tanımlarken verilerin türünü ve dizi uzunluğunu da tanımlayabiliyoruz.
Aşağıdaki örnekte **i32** dizi içindeki değerlerin **veri türü**, **5** ise **dizinin uzunluğu**dur.
```sh
fn main(){
    let numbers:[i32;5] = [1, 2, 3, 4, 5];
}
```
Aynı değerlere sahip bir dizi oluşturmak istediğimiz zaman aşağıdaki gibi bir tanımlama yapıyoruz.
Burada default değerimiz **1**, dizi uzunluğumuz **10**. Yani 10 tane 1 değerimiz vardır.
```sh
fn main(){
    let numbers = [1; 10];
}
```
Dizi elemanlarına erişmek için köşeli parantezler içerisinde index numarasını verebiliriz.
```sh
fn main(){
    let numbers = [1, 2, 3];
    println!("{}",numbers[1]);
}
```