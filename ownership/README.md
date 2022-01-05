# Ownership
ownership kavramı Rust dilinde temel bir özelliği sahiptir. 

## Ownership Rules
Rust diline göre belli başlı ownership kuralları vardır.
  - Her değerin sahibi kendi değişkenidir.
  - Sadece bir tane ownership olabilir.
  - Scope dışına çıkıldığında değer düşer.

Rust derleyicisi her bir değerin tek bir sahibinin olduğunun kontrolünü yapar. Bu sayede yarattığınız nesneyi bir başka objeye atadığınızda nesnenin sahipliği atanan objeye geçer. Nesneyi atadığınız kapsam içerisinde bir daha o değişken kullanılmaz. Çünkü artık o nesnenin sahipliği atama yapılan değişkenin kapsamında değildir.

```sh
fn main() {
    let text = String::from("Hello, World");
    let second_text = text;
    println!("{}", text);
}
```
Yukarıda ki örnek kodumuzu çalıştırdığımız zaman hata alacağız.
`text` değerini `second_text` adlı değişkene atadığımız zaman artık **text** değerindeki verinin sahipliği **second_text** adlı değişkene aktarılmış oldu. Bu nedenle **text** değerini ekrana basmaya çalıştığımız da hata almaktayız.
```sh
error[E0382]: borrow of moved value: `text`
 --> src\main.rs:5:20
  |
2 |     let text = String::from("Hello, World");
  |         ---- move occurs because `text` has type `String`, which does not implement the `Copy` trait
3 |     let second_text = text;
  |                       ---- value moved here
4 | 
5 |     println!("{}", text);
  |                    ^^^^ value borrowed here after move
```
# Borrowing 
Borrowing konusu, bir değerin başka bir değişkene ödünç verilmesi olarak tanımlayabiliriz. Değeri ödünç alan nesne; değer ile işe bittiği zaman asıl sahibine geri verir. Ödünç verme işleminin iki kuralı vardır.
  - İstediğimiz sayıda değiştirilemez (immutable) referans verilebilir.
  - Değiştirilebilir (mutable) referans ise sadece bir tane verilebilir.
İlk olarak yukarıda hata almış olduğumuz kodu düzenleyerek başlayalım.

Burada **text** değişkeninden veriyi ödünç almak için `&` işaretini kullanıyoruz. **second_text** değişkenine atama yaparken **&text** şeklinde yazarsak artık **text** değişkenini print ederken hata almayacağız. Çünkü verinin asıl sahibi hala **text** değişkeni.
```sh
fn main() {
    let text = String::from("Hello, World");
    let second_text = &text;
    println!("{}", text);
}
```
Başka bir örnek;
Bu örnekte fonksiyon parametresini **&String** olarak tanımladık. Yani referans değer istiyoruz şeklinde çevirebiliriz.
```sh
fn main() {
    let text = String::from("Hello, World");
    let size = get_length(&text);
    println!("{}", size);
}
fn get_length(string : &String) -> usize {
    string.len()
}
```
Mutable değişkenler ile örnek;
Dikkat etmemiz gereken noktalar **&mut** tanımlamaların bulunduğu yerlerdir.
```sh
fn main() {
    let mut second_text = String::from("Hello, Second Text");
    let new_text = add_string(&mut second_text);
    println!("{}",new_text);
}

fn add_string(string : &mut String) -> &mut String {
     string.push_str(", Hello Rusty");
     return string
}
``` 