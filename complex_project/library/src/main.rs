use school::lessons::Lesson;

fn main() {
    let l1 = Lesson::new_first("ders".to_string());

    println!("{:?}", l1);
}


#[cfg(test)]
mod test {
    #[test]
    fn library_test() {
        println!("library::main()");
        println!("first: {:?}", super::Lesson::new_first("this is first library".to_string()));
        println!("second: {:?}", super::Lesson::new_second("this is second library".to_string()));
    }
}