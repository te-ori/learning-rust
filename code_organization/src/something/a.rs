pub struct A {
    pub a: i32,
}

#[cfg(test)]
mod test {
    #[test]
    fn exec() {
        println!("crate::something::a");
    }
}