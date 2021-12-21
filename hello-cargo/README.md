# Hello Cargo
**Cargo**, Rust programlama dilinin build sistemi ve paket yöneticisidir.
Rust üzerinde kodumuzu build etmemizi ve başka kütüphaneleri projelerimize dahil etmemizi sağlar. 
Eğer Rust dilini kendi sitesinden yüklediyseniz otomatik olarak kurulu gelmektedir.

`cargo new projeadi` sözdizimi ile terminal üzerinden yeni bir proje oluşturabiliyoruz.
```sh
cargo new hello-with-cargo
```
Proje oluşturulduktan sonra "hello-with-cargo" klasörü içerisinde `Cargo.toml` dosyası ve `src` klasörü oluşturuldu. 
**src** klasörü içerisinde otomatik olarak **main.rs** dosyamız ve içerisinde println! kodu bulunmakta.
# Cargo.toml
`Cargo.toml`, Rust dilinin konfigürasyon dosyasıdır.
Cargo.toml dosyasını incelediğimizde;
**[package]** altındaki parametreler projemizin bilgilerini içermektedir. 
**[dependencies]** altında ise projemize entegre etmek istediğimiz kütüphanelerin isimlerini yazacağız. 
```sh
[package]
name = "hello-with-cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

# Cargo ile Build Alma ve Çalıştırma
"hello-with-cargo" isimli klasör içerisine geçerek cargo ile build alacağız. `cargo build` ile projemizin build işlemini başlatıyoruz.
```sh
cargo build
```
Compile işlemi bittikten sonra klasör içerisinde bir dosya ve klasör oluşturuldu.
`Cargo.lock` dosyası; projenin bağımlılıklarını kontrol eder. Sonradan düzenlenmemesi gereken bir dosyasıdır.
Şuan bu projede herhangi bir dependencies olmadığı için şimdilik geçiyoruz.

`target/debug/` klasörü içerisinde projenin çalıştırılabilir hali bulunmaktadır.
Projeyi iki şekilde çalıştırabiliriz.
İlk olarak;
```sh
./target/debug/hello-with-cargo
```
İkinci olarak;
```sh
cargo run
```

`cargo build` koduna ek olarak `cargo check` ile de projeyi derleyebiliriz. Fakat `cargo check` projeyi derledikten sonra çalıştırılabilir bir kod üretmemektedir. Bu yüzden `cargo build` kodundan hızlı çalışmaktır. 