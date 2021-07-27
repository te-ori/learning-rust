#[allow(dead_code)]
fn main() {
    shared_reference();
}

#[derive(Debug)]
struct Shared {
    age: u8,
    hobbies: Vec<i8>,
}

fn shared_reference() {
    let mut shr = Shared {
        age: 10,
        hobbies: vec![1, 2, 3],
    };
    
    let r_shr_1 = &shr;
    let r_shr_2 = &shr;
    let r_r_shr_1 = &r_shr_1;
    let r_shr_hobbies = &shr.hobbies;
    // let mr_shr_1 = &mut shr; // ~~ cannot borrow `shr.hobbies` as mutable because it is also borrowed as immutable
    // let r_shr_hobbies = &mut shr.hobbies; // ~~ cannot borrow `shr.hobbies` as mutable because it is also borrowed as immutable
    
    println!("r_shr_1 {:?}", r_shr_1);
    println!("r_shr_2 {:?}", r_shr_2);
    println!("r_r_shr_1 {:?}", r_r_shr_1);
    println!("r_shr_hobbies {:?}", r_shr_hobbies);
    
    println!();
    
    // let mr_shr_1 = &mut shr;
    println!("r_shr_1 {:?}", r_shr_1);
    
    // let mr_shr_1 = &mut shr; // ~~ cannot borrow `shr` as mutable more than once at a time
    // println!("r_shr_1 {:?}", r_shr_1);
    
    let mut mr_shr_1 = &mut shr; // ~~ cannot borrow `shr` as mutable more than once at a time

    // let mr_shr_2 = &mut shr; // ~~ cannot borrow `shr` as mutable more than once at a time
    // let r_shr_3 = &shr; // ~~ cannot borrow `shr` as immutable because it is also borrowed as mutable
    // let r_shr_hobbies = &shr.hobbies; // ~~ cannot borrow `shr.hobbies` as immutable because it is also borrowed as mutable
    
    println!("mr_shr_1 {:?}", mr_shr_1);
    
    let mr_mr_shr_1 = &mut mr_shr_1;
    // println!("mr_shr_1 {:?}", mr_shr_1); // ~~ cannot borrow `mr_shr_1` as immutable because it is also borrowed as mutable
    println!("mr_mr_shr_1 {:?}", mr_mr_shr_1);
    let r_mr_shr_1 = &mr_shr_1;
    println!("mr_mr_shr_1 {:?}", mr_mr_shr_1);
    println!("mr_shr_1 {:?}", mr_shr_1);
    
}

fn some() {
    let mut v = vec![1, 2, 3];
    println!("v{:?}", v);

    v.push(10);
    println!("v{:?}", v);

    let vv = v;
    println!("vv{:?}", vv);
    // vv.push(45); // ~~ cannot borrow `vv` as mutable, as it is not declared as mutable
    v = vv;
    println!("v{:?}", v);

    v.push(20);
    println!("v{:?}", v);
}