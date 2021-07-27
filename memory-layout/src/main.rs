use std::mem::size_of;


struct X {
    a: u8,
}

struct Y {
    a: u8,
    b: u8,
}

struct Z {
    a: u8,
    b: u8,
    c: u8,
}

struct Z2 {
    a: u8,
    b: u16,
}

struct A {
    a: u8,
    b: u32,
}

struct B {
    a: u16,
    b: u32,
}

struct C {
    a1: u8,
    a2: u8,
    b: u32,
}

struct D {
    a1: u8,
    a2: u8,
    b: u32,
}



#[repr(packed)]
struct X_Packed {
    a: u8,
}

#[repr(packed)]
struct Y_Packed {
    a: u8,
    b: u8,
}

#[repr(packed)]
struct Z_Packed {
    a: u8,
    b: u8,
    c: u8,
}

#[repr(packed)]
struct Z2_Packed {
    a: u8,
    b: u16,
}

#[repr(packed)]
struct A_Packed {
    a: u8,
    b: u32,
}

#[repr(packed)]
struct B_Packed {
    a: u16,
    b: u32,
}

#[repr(packed)]
struct C_Packed {
    a1: u8,
    a2: u8,
    b: u32,
}

#[repr(packed)]
struct D_Packed {
    a1: u8,
    a2: u8,
    b: u32,
}

fn main() {
    println!("X:  {}, X_Packed:  {}", size_of::<X>(), size_of::<X_Packed>());
    println!("Y:  {}, Y_Packed:  {}", size_of::<Y>(), size_of::<Y_Packed>());
    println!("Z:  {}, Z_Packed:  {}", size_of::<Z>(), size_of::<Z_Packed>());
    println!("Z2: {}, Z2_Packed: {}", size_of::<Z2>(), size_of::<Z2_Packed>());
    println!("A:  {}, A_Packed:  {}", size_of::<A>(), size_of::<A_Packed>());
    println!("B:  {}, B_Packed:  {}", size_of::<B>(), size_of::<B_Packed>());
    println!("C:  {}, X_Packed:  {}", size_of::<X>(), size_of::<X_Packed>());
    println!("D:  {}, D_Packed:  {}", size_of::<D>(), size_of::<D_Packed>());
}
