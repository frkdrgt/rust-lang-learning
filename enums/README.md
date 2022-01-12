# Enums
Enumlar bir çok programlama dilindeki gibi, değişkenlerin alabileceği değerlerin sabit olduğu durumlarda programı daha okunabilir hale getirmek için kullanılır. 
`enum` anahtar sözcüğü ile tanımlanır.
```sh
enum NetworkType{
    TCP
    UDP,
}
```
## Enum Değerleri
Her iki değerde aynı namespace de oldukları için **::** ile atamak istediğimiz değeri seçiyoruz.
```sh
enum NetworkType {
    TCP,
    UDP
}
fn main() {
    let network = NetworkType::TCP;
}
```
Fonksiyonlarda parametre olarak kullanabiliriz.
```sh
enum NetworkType {
    TCP,
    UDP
}
fn main() {
    set_network_type(NetworkType::UDP);
}
fn set_network_type(network_type : NetworkType) {}
```
Struct yapılarında kullanabiliriz.
```sh
enum NetworkType {
    TCP,
    UDP
}
fn main() {
    let server = Network {
        network_type: NetworkType::TCP,
        server_ip: String::from("127.0.0.1")
    };
}
struct Network {
    network_type : NetworkType,
    server_ip : String
}
```
## match operator
match operatörü diğer programlama dillerindeki switch case yapısına benzerdir. match içerisine verdiğimiz modeli,  verilen şarta uygun değer olup olmadığına bakar ve buna göre işlem yapar.
Bir fonksiyon içerisinde örnek olarak match karşılaştırılması aşağıdaki gibidir.
```sh
fn correct_network_type(network_type: NetworkType) -> bool {
    match network_type {
        NetworkType::TCP => true,
        NetworkType::UDP => false
    }
}
```
Bir başka kullanım türü;
```sh
fn correct_network_type(network_type: NetworkType) -> bool {
    match network_type {
        NetworkType::TCP => {
            println!("TRUE VALUE");
            true
        },
        NetworkType::UDP => false
    }
}
```