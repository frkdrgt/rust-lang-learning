use std::sync::mpsc::Receiver;

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

impl Image {
    fn create(size : u32) -> Image {
        Image {
            width:size,
            height:size
        }
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

    let image = Image::create(32);
    
    println!("{} {}", image.width,image.height);
}
