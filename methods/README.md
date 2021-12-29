# Methods
Methodlar, fonksiyonlara benzer yapılardır. `fn` keywordü ile tanımlanırlar. Parametresiz ve parametreli olabilirler ve geriye değer döndürebilirler.
Burada methodlarımızı **impl** yapısı içerisinde tanımlamaya başlıyoruz.
Ancak methodlar bir struct yapı içerisinde tanımlanır ve ilk parametreleri her zaman `self` tanımlamasıdır.
`self` parametresinin başındaki **&** işaretini ownership konusunda değineceğiz.
```sh
struct Image { 
    width: u32,
    height: u32
}

impl Image {
    fn resize(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let image = Image {
        width:100,
        height:200
    };
    let new_size = image.resize();

    println!("{}", new_size);
}
```
## Birden Fazla Parametreli Method
**impl** içerisine parametreli bir şekilde **is_bigger** adında ikinci fonksiyonumuzu tanımladık.
```sh
struct Image { 
    width: u32,
    height: u32
}

impl Image {
    fn resize(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, other : &Image) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let image = Image {
        width:100,
        height:200
    };
    let new_size = image.resize();
    println!("{}", new_size);

    let other_image = Image {
        width: 50,
        height: 50
    };

    let is_bigger = image.is_bigger(&other_image);

   println!("{}", is_bigger);
}
```
Yukarıda ki örneğe ek olarak; impl bloğunu ayırarakta methodlar oluşturabiliriz. Örneğin;
```sh
struct Image { 
    width: u32,
    height: u32
}

impl Image {
    fn resize(&self) -> u32 {
        self.width * self.height
    }

}

impl Image {
    fn is_bigger(&self, other : &Image) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let image = Image {
        width:100,
        height:200
    };
    let new_size = image.resize();
    println!("{}", new_size);

    let other_image = Image {
        width: 50,
        height: 50
    };

    let is_bigger = image.is_bigger(&other_image);

   println!("{}", is_bigger);
}
```
## Associated Fonksiyonlar
**impl** blokları içerisine tanımladığımız fonksiyonlar aynı zamanda **associated functions** olarak geçmektedir. Associated functions oluşturmak için fonksiyonu tanımlarken **self** parametresini tanımlamamaktır.
Bu örnekte fonksiyonumuzu kullanırken **::create** tanımlaması ile çağırıyoruz.
```sh
impl Image {
    fn create(size : u32) -> Image {
        Image {
            width:size,
            height:size
        }
    }
}

fn main() {
    let image = Image::create(32);
    
    println!("{} {}", image.width,image.height);
}
```