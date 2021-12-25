# Structs
Struct veya structure, anlamlı bir grup oluşturan birbiriyle ilişkili birden çok değeri adlandırmanıza ve paketlemenize olanak tanıyan özel bir veri türüdür. Nesne yönelimli bir dile aşina iseniz, yapı bir nesnenin veri öznitelikleri gibidir. 
Rust dilinde `struct` anahtar kelimesi ile tanımlanır. Struct isimlendirmesi anlamlı bir şekilde yapılacak göreve göre isimlendirilmelidir. Daha sonra süslü parantezler içerisinde field dediğimiz veri parçalarını oluşturmaya başlıyoruz.
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}
```
Tanımladığımız struct yapısını kullanmak için ondan bir örnek oluşturmamız gerekmektedir.
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let admin = User {
        first_name: String::from("Faruk"),
        last_name: String::from("Durgut"),
        age: 28,
        active: true
    };

    println!("{}", admin.first_name);
}
```
Tanımladığımız **admin** yapısından daha sonra herhangi bir değeri değiştirmek istediğimiz zaman değişkeni **mut** keywordü ile tanımlamamız gerekmektedir.
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let mut admin = User {
        first_name: String::from("Faruk"),
        last_name: String::from("Durgut"),
        age: 28,
        active: true
    };

    admin.first_name = String::from("Omer Faruk");
    println!("{}", admin.first_name);
}
```
Şimdi bir fonksiyon kullanarak User struct yapısından kullanıcı oluşturalım.
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let moderator = create_user(String::from("Faruk"),String::from("Durgut"));
    println!("{} {}", moderator.first_name, moderator.last_name);
}

fn create_user (first_name: String, last_name: String) -> User {
    User {
        first_name: first_name,
        last_name: last_name,
        age: 28,
        active: false
    }
}
```
Bu örnekte fonksiyon **User** tipi döndüren bir fonksiyon yazdık. Burada parametrelere dikkat edersek **first_name** ve **last_name** parametleri **User struct** parametleri ile aynı isimlendirmeye sahip ve bu biraz garip görünüyor.
Burada Rust dilinde **field init shorthand syntax** yapısını kullanabiliriz. Bu yapı sayesinde aynı isimlendirmeye sahip parametleri tekrar yazmaktan kaçınıyoruz.
Yani fonksiyon kodumuzu aşağıdaki gibi güncelleyebiliriz. 
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let moderator = create_user(String::from("Faruk"),String::from("Durgut"));
    println!("{} {}", moderator.first_name, moderator.last_name);
}

fn create_user (first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        age: 28,
        active: false
    }
}
```
Gördüğünüz gibi **first_name** ve **last_name** parametlerini atama işlemi yapmadan isim tekrarlanmasını önlemiş olduk.

Oluşturmuş olduğumuz User structından bir örnek alarak yeni bir User oluşturabiliriz.
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let moderator = create_user(String::from("Faruk"),String::from("Durgut"));
    println!("{} {}", moderator.first_name, moderator.last_name);

    let copy_moderator = User {
        first_name: String::from("Copy Moderator"),
        ..moderator
    };
    println!("{} {}", copy_moderator.first_name, copy_moderator.last_name);
}

fn create_user (first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        age: 28,
        active: false
    }
}
```
Burada **copy_moderator** içerisinde sadece first_name güncelledik ve **..moderator** sözdizimi ile bir üstteki tanımladığımız **moderator User**ının tüm bilgileri ile aynı şekilde oluşturduk.
Bir başka yöntem ise;
```sh
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let mut admin = User {
        first_name: String::from("Faruk"),
        last_name: String::from("Durgut"),
        age: 28,
        active: true
    };

    admin.first_name = String::from("Omer Faruk");
    println!("{}", admin.first_name);


    let moderator = create_user(String::from("Faruk"),String::from("Durgut"));
    println!("{} {}", moderator.first_name, moderator.last_name);

    let another_copy_moderator = User {
        first_name: moderator.first_name,
        last_name: moderator.last_name,
        age: 30,
        active:false
    };

    println!("{} {}", another_copy_moderator.first_name, another_copy_moderator.last_name);
}

fn create_user (first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        age: 28,
        active: false
    }
}
```
## Tuple Struct
Rust dilinde tuple yapılarına benzer struct yapıları oluşturabiliriz. Bunlara tuple struct denilmektedir. Sadece veri tiplerinin tanımlandığı içerisinde field olmayan struct yapılarıdır.
```sh
struct Color(i32, i32, i32);
fn main() {
    let black = Color(0,0,0);
    println!("{} {} {}",black.0,black.1,black.2);
}
```
Tuple struct, tuple gibi davrandığı için değerlere ulaşılırken indexleri üzerinden erişebiliyoruz.
## Unit-Like Structs
Rust dilinde field kullanmadan tanımlanan structlara **unit-like structs** denilmektedir. Bir tür üzerinde veri depolamak istemediğimiz zamanlarda kullanmak için uygun olabilir.
```sh
struct AlwaysEqual;
fn main() {
    let condition = AlwaysEqual;
}
```
