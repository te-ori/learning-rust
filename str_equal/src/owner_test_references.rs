pub fn string_test() {
    let s1 = String::from("Hello");
    println!("s1 {}", s1);

    let s2 = s1; // ! value moved here
    // println!("s1 {}", s1); //! value borrowed here after move
    println!("s2 {}", s2);

    let s3 = s2 + " World";  // ! `s2` moved here
    // println!("s2 {}", s2); //! value borrowed here after move
    println!("s3 {}", s3);

    let mut s4 = s3.clone();
    s4.push_str("!");
    println!("s4 {}", s4);

    let s5 = s4 + "!"; // ! `s4` moved here
    // println!("s4 {}", s4); //! value borrowed here after move
    println!("s5 {}", s5);

    let s6  = &s5;
    println!("s5 {}", s5);
    println!("s6 {}", s6);

    let s7 = s5;// ! `s4` moved here
    // println!("s5 {}", s5); //! value borrowed here after move
    println!("s7 {}", s7);
}

pub fn test_vec() {
    let v1 = vec![1, 2, 3, 4];
    println!("v1: {:?}", v1);

    let v2 = v1;// ! `v1` moved here
    // println!("v1 {}", v1); //! value borrowed here after move
    println!("v2: {:?}", v2);

    let v3_1 = vec![5, 6];
    let v3_2 = vec![7, 8];
    // let v4 = v3.append(v2); // ! expected mutable reference, found struct `std::vec::Vec`
    // let v4 = v3.extend(v2); // ! cannot borrow `v3` as mutable, as it is not declared as mutable

    let mut v4 = vec![0];
    // v4.append(v3);//! expected mutable reference, found struct `std::vec::Vec`
    // v3.append(&v4); // ! expected mutable reference `&mut std::vec::Vec<{integer}>` found reference `&std::vec::Vec<{integer}>`
    v4.extend(v3_1); // `v3_1` borrowed here
    // println!("v3_1 {}", v3_2); // ! value borrowed here after move
    println!("v4: {:?}", v4);

    v4.extend(&v3_2);
    println!("v3_2: {:?}", v3_2);
    println!("v4: {:?}", v4);

    let v5 = vec![1, 2, 3];
    let mut v6 = vec![0];

    v6.extend(v5.iter());
    println!("v5: {:?}", v5);
    println!("v6: {:?}", v6);
    
    let v7 = vec![1, 2, 3];
    let mut v8 = vec![0];

    // v8.append(v7); // ! expected mutable reference, found struct `std::vec::Vec`
    // v8.append(&v7); // ! types differ in mutability
    // v8.append(&mut v7); // ! cannot borrow `v7` as mutable, as it is not declared as mutable
    // v8.append(v7.iter()); // ! mismatched types expected mutable reference, found struct `std::slice::Iter`

    let mut v9 = vec![1, 2, 3];
    let mut v10 = vec![0];
    let mut v11 = vec![4, 5];
    let mut v12 = vec![6, 7];

    // v10.append(v9); // ! mismatched types expected mutable reference, found struct `std::vec::Vec`
    v10.append(&mut v9);
    println!("v9: {:?}", v9);
    println!("v10: {:?}", v10);

    v10.extend(v11); // `v11` moved here;
    // println!("v11: {:?}", v11); // ! value borrowed here after move
    println!("v10: {:?}", v10);

    v10.extend(&v12);
    println!("v12: {:?}", v12);
    println!("v10: {:?}", v10);
}

#[derive(Debug)]
struct St1;

pub fn test_unit_structs() {
    let s_st1_1 = St1;
    println!("s_st1_1: {:?}", s_st1_1);

    let s_st1_2 = s_st1_1; // ! `s_st1_1` moved here
    // println!("s_st1_1: {:?}", s_st1_1); // ! value borrowed here after move
    println!("s_st1_2: {:?}", s_st1_2);

    let s_st1_3 = &s_st1_2;
    println!("s_st1_2: {:?}", s_st1_2);
    println!("s_st1_3: {:?}", s_st1_3);
}

#[derive(Debug)]
struct StI32(i32);

pub fn test_tuple_struct_with_primitive() {
    let n1: i32 = 1;
    let s_st_i32_1 = StI32(n1);
    println!("n1: {:?}", n1);
    println!("s_st_i32_1: {:?}", s_st_i32_1);

    let s_st_i32_2 = StI32(n1);
    println!("n1: {:?}", n1);
    println!("s_st_i32_2: {:?}", s_st_i32_2);

    let mut n2: i32 = 2;
    let s_st_i32_3 = StI32(n2);
    println!("n2: {:?}", n2);
    println!("s_st_i32_3: {:?}", s_st_i32_3);

    n2 += 1;
    let s_st_i32_4 = StI32(n2);
    println!("n2: {:?}", n2);
    println!("s_st_i32_4: {:?}", s_st_i32_4);

    let s_st_i32_5 = s_st_i32_4; // ! `s_st_i32_4` moved here 
    println!("n2: {:?}", n2);
    // println!("s_st_i32_4: {:?}", s_st_i32_4); // ! value borrowed here after move
    println!("s_st_i32_5: {:?}", s_st_i32_5);

}

pub fn test_tuple_struct_with_reference() {
    // let s_st_str_1 = StStr("Hello"); // ! mismatched types; expected struct `std::string::String`, found `&str`
    let s_st_str_1 = StStr("Hello".to_string());
    println!("s_st_str_1: {:?}", s_st_str_1);

    let s2 = "Hello".to_string();
    let s_st_str_2 = StStr(s2); // `s2` moved here
    // println!("s1: {:?}", s2); // ! value borrowed here after move
    println!("s_st_str_2: {:?}", s_st_str_2);

    let s_st_str_3 = s_st_str_1; // `s_st_str_1` moved here
    // println!("s_st_str_1: {:?}", s_st_str_1); // ! value borrowed after move
    println!("s_st_str_3: {:?}", s_st_str_3);

    let s3 = "Hello".to_string();
    let s_st_str_4 = StStr(s3.clone());
    println!("s3: {}" ,s3);
    println!("s_st_str_4: {:?}", s_st_str_4);

}

#[derive(Debug)]
struct StStr(String);

#[derive(Debug)]
struct StPropI32 {
    p1: i32
}

#[derive(Debug)]
struct StPropStr {
    p1: String
}

pub fn test_struct_with_string_prop() {
    let s1 = "Hello".to_string();
    let s_str_prop_str_1 = StPropStr { p1: s1}; // `s1` moved here
    // println!("s1: {}", s1); // ! `s1` borrowed after moved
    println!("s_str_prop_str_1: {:?}", s_str_prop_str_1);

    let s_str_prop_str_2 = s_str_prop_str_1; // ! `s_str_prop_str_1` moved here
    // println!("s_str_prop_str_1: {:?}", s_str_prop_str_1); // ! `s_str_prop_str_1` borrowed after move
    println!("s_str_prop_str_2: {:?}", s_str_prop_str_2);
    println!("s_str_prop_str_2.p1: {:?}", s_str_prop_str_2.p1);

    let s2 = s_str_prop_str_2.p1;  // Not only ownership of `s_str_prop_str_2.p1` moved, ownership of `s_str_prop_str_2` completely moved here.
    // println!("s_str_prop_str_2: {:?}", s_str_prop_str_2); // ! value borrowed here after partial move
    // println!("s_str_prop_str_2.p1: {:?}", s_str_prop_str_2.p1); // ! value borrowed here after move
    println!("s2: {:?}", s2);

}

#[derive(Debug)]
struct StPropStrNI32 {
    p1: String,
    p2: i32
}

pub fn test_struct_with_string_n_i32_prop() {
    let s1 = "Hello".to_string();
    let n1: i32 = 1;
    let s_str_n_i32_prop_1 = StPropStrNI32 { p1: s1, p2 : n1}; // `s1` moved here
    // println!("s1: {}", s1); // ! `s1` borrowed after moved
    println!("n1: {:?}", n1);
    println!("s_str_n_i32_prop_1: {:?}", s_str_n_i32_prop_1);
    println!("s_str_n_i32_prop_1.p1: {:?}", s_str_n_i32_prop_1.p1);
    println!("s_str_n_i32_prop_1.p2: {:?}", s_str_n_i32_prop_1.p2);

    let n2 = s_str_n_i32_prop_1.p2 + 1; // moving does not occure because `s_str_n_i32_prop.p1` primitive so it does not *move*, it does copied.
    println!("n2: {:?}", n2);
    println!("s_str_n_i32_prop_1: {:?}", s_str_n_i32_prop_1);
    println!("s_str_n_i32_prop_1.p1: {:?}", s_str_n_i32_prop_1.p1);
    println!("s_str_n_i32_prop_1.p2: {:?}", s_str_n_i32_prop_1.p2);
    
    let s2 = s_str_n_i32_prop_1.p1; // moving occurs because data part of `s_str_n_i32_prop.p2` resides in heap so ownership moved. 
    println!("s2: {:?}", s2);
    // println!("s_str_n_i32_prop_1: {:?}", s_str_n_i32_prop_1); // ! value borrowed here after partial move 
    // println!("s_str_n_i32_prop_1.p1: {:?}", s_str_n_i32_prop_1.p1); // ! move occurs because `s_str_n_i32_prop_1.p1` has type `std::string::String`, which does not implement the `Copy` trait
    println!("s_str_n_i32_prop_1.p2: {:?}", s_str_n_i32_prop_1.p2); // it works so it means when partial moving occurs only heaped members move. Strictly speaking members which do not implemented `Copy` trait move. Others dont. 
}

