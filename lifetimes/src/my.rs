fn main() {
    let mut l1 = Lol::default();

    println!("{}", l1);

    l1.add_lucky_number(5);
    println!("{}", l1);

    l1.add_book("test".to_string());
    println!("{}", l1);

    let b = "bb".to_string();
    l1.add_book(b);
    println!("{}", l1);

}

struct Lol {
    age: u8,
    name: String,
    books: Vec<String>,
    lucky_numbers: Vec<i32>
}

impl Lol {
    pub fn default() -> Lol{
        Lol {
            age: 0,
            name: "unnamed".to_string(),
            books: Vec::new(),
            lucky_numbers: Vec::new()
        }
    }

    pub fn add_lucky_number(&mut self, n: i32) {
        self.lucky_numbers.push(n);
    }

    pub fn add_book(&mut self, book: String) {
        self.books.push(book);
    }
}

impl std::fmt::Display for Lol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}, {:?}", self.name, self.age, self.lucky_numbers, self.books)
    }
}