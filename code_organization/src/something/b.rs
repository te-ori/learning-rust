pub struct B {
    pub b: i32,
}

mod test {
    #[test]
    fn exec() {
        println!("crate::something::b");
    }
}