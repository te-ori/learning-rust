fn main() {
    _without_move();
    _with_move();
    _without_move_more_than_one();
    _with_move_more_than_one();
}

fn _without_move() {
    println!("\nWithout Move\n");
    let mut v = vec![1, 2, 3];
    println!("v {:?}", v);
    
    let mut add_to_v = |n| v.push(n);
    add_to_v(5);
    println!("v {:?}", v);
}

fn _with_move() {
    println!("\nWith Move\n");
    let mut v = vec![1, 2, 3];
    println!("v {:?}", v);
    
    let add_to_v = move |n| {
        v.push(n); 
        v
    };

    let nv = add_to_v(6);

    // println!("v {:?}", v);
    // ~~                 ^ value borrowed here after move

    println!("nv: {:?}", nv);
}

enum Operation {
    Sum,
    Subtr,
}

fn _without_move_more_than_one() {
    println!("\nWithout Move More Than One\n");

    use Operation::*;

    let v1 = vec![1,2,3,4];
    let v2 = vec![2,4,8,16];
    let v3 = vec![0, 1, 2, 3];

    let operation_over_vecs = |op: Operation| {
        match op {
            Sum =>  v1.iter().zip(v2.iter()).map(|(e1, e2)| e1 + e2).collect(),
            Subtr =>  v1.iter().zip(v2.iter()).map(|(e1, e2)| e1 - e2).collect(),
        }
    };

    let n:Vec<i32> = operation_over_vecs(Sum);
    let m:Vec<i32> = operation_over_vecs(Subtr);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("n {:?}", n);
    println!("m {:?}", m);
    println!("v3 {:?}", v3);
}

fn _with_move_more_than_one() {
    println!("\nWith Move More Than One\n");
    
    use Operation::*;

    let v1 = vec![1,2,3,4];
    let v2 = vec![2,4,8,16];
    let v3 = vec![0, 1, 2, 3];

    let operation_over_vecs = move |op: Operation| {
        match op {
            Sum =>  v1.iter().zip(v2.iter()).map(|(e1, e2)| e1 + e2).collect(),
            Subtr =>  v1.iter().zip(v2.iter()).map(|(e1, e2)| e1 - e2).collect(),
        }
    };

    let n:Vec<i32> = operation_over_vecs(Sum);
    // !             ----------------------------------- `operation_over_vecs` moved
    // ~~ let m:Vec<i32> = operation_over_vecs(Operation::Sum);
    // ~~                  ^^^^^^^^^^^^^^^^^^^ value used here after move
    // ~~ println!("{:?}", v1);
    // ~~               ^^ value borrowed here after move
    // ~~ println!("{:?}", v2);
    // ~~               ^^ value borrowed here after move
    println!("n {:?}", n);
    println!("v3 {:?}", v3);
}