# Değişkenler
Rust içerisinde tanımladığımız değişkenler varsayılan olarak immutable(değiştirilemez) tanımlanır. 
Bu Rust dilinin güvenlik için sağladığı bazı avantajlardan biridir.
Fakat tabii ki değişkenleri mutable(değişebilir) olarak tanımlayabiliyoruz.

Immutable tanımlanan değişkenlerin içerisinde ki veri daha sonradan değiştirilemez.
Şimdi `cargo new variables` ile yeni bir proje oluşturuyoruz ve `src/main.rs` dosyasında main fonksiyonu içerisine değişkenleri tanımlayalım.
```sh
fn main(){
 let x = 5;
 println!("x değeri: {}", x);
}
```
**let** sözdizimi ile değişken tanımlaması yapılıyor. Burada x değişkenimiz otomatik olarak immutable tanımlandı.
Kodumuzu aşağıdaki gibi güncelledikten sonra `cargo build` çalıştırdığımız zaman **^^^^^ cannot assign twice to immutable variable** şeklinde hata dönecektir. Yeni bir değer atamamıza izin verilmedi.
```sh
fn main(){
 let x = 5;
 println!("x değeri: {}", x);
 x = 6;
 println!("x değeri: {}", x);
}
```
```sh
  |
  |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
  |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

Bu problemi gidermek için değişkeni **let mut x = 5** şeklinde tanımlamamız gerekir.
Şimdi `cargo build` ve `cargo run` çalıştıralım. 
```sh
fn main(){
 let mut x = 5;
 println!("x değeri: {}", x);
 x = 6;
 println!("x değeri: {}", x);
}
```
```sh
x değeri: 5
x değeri: 6
```
# Constants
Rust dilinde sabit değişmeyen değişken tanımlamak için; **const** anahtar sözcüğü kullanılır.
Değişkenlerden farklı olarak her zaman immutable(değiştirilemez) olmasıdır.
**const** tanımlamasında veri tipini belirtmemiz gerekmektedir.
```sh
fn main() {
    const PAGE_COUNT : i32 = 100; 
}
```