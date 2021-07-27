trait First {
    fn get(&self)
    u i32,
}

struct Sfi;

impl First for Sfi {
    u = 20;
    fn get(&self) {
        println!("self.u {}", self::u);
    }
}

pub fn use_sfi() {
    let sfi = Sfi;

    sfi.get();
}