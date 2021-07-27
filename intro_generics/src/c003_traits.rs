struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub fn use_it() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty; //! `value used here after moverustc(E0382)`
    // null; //! `value used here after moverustc(E0382)`
}