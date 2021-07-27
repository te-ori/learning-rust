pub mod a;
pub mod b;

#[cfg(test)]
mod test {
    #[test]
    fn exec() {
        println!("crate::something");
    }
}