# Hello, Rust

İlk olarak **hello-world** adında bir klasör oluşturduk. İçerisine main.rs (rust dilinin uzantası .rs) adında bir dosya oluşturduk.
Dosyamızın içerisine ilk giriş kodumuzu yazıyoruz. 
```sh
fn main(){
    println!("Hello, Rust");
}
```

**fn** sözdizimi ile Rust programlama dilinde fonksiyonlar oluşturuluyor. **fn main()** ise özel bir fonksiyondur. Rust programında her zaman çalışan ilk koddur.

Fonksiyon içerisinde bulunan `println!` ekrana veri bastırabiliyoruz. Buradaki **!(ünlem)** işareti Rust dilinde makroları tanımlanır. Yani println! yaptığımızda bir makro çağırmış oluyoruz. Makrolara ilerleyen kısımlarda tekrardan bakacağız.

# Projeyi Derleme ve Çalıştırma
VSCode üzerinde terminalden (yada normal terminal) `rustc main.rs` ile projemizi derleyebiliriz. (`rustc` Rust Compiler)
Compile işleminden sonra terminalde `./main` diyerek projemizi çalıştırabiliyoruz.
```sh
rustc main.rs
./main
```
```sh
Hello, Rust
```