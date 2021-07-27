struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("dropped {}", self.0);
    }
}

fn main() {
    // let n = 0;
    // let lol = || println!("lol {}", n);
    // lol();
    // // drop1();
    // drop2();
let factor = 5;
    let m = |i: i32| i * 5;
    apply(10, m);
}

fn apply(i: i32, f: fn(i32) ->i32) {
    println!("applied: {}", f(i));
}

fn drop1() {


    let _a = S(1);
    let _b = S(2);
    let _c = S(3);
    {
        let _d = S(4);
        let _e = S(5);
        let _f = S(6);
        println!("INNER");
    }
    println!("OUTER");
}
fn drop2() {
    let _ = S(1);
    let _ = S(2);
    let _ = S(3);
    {
        let _ = S(4);
        let _ = S(5);
        let _ = S(6);
        println!("INNER");
    }
    println!("OUTER");
}
