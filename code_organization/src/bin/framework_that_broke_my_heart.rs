extern crate code_organization;

use code_organization::something::a::*;
use code_organization::something::b::*;

fn main() {
    let first = A { a: 42 };
}

mod test {
    #[test]
    fn exec() {
        println!("crate");
    }
}