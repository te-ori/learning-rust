pub fn primitives_in_array() {
    let ai32 = [0;10];

    // println!("with i indices");
    // for i in 0..ai32.len() {
    //     println!("a[{}]: {}", i, ai32[i]);
    // }

    // println!("with enumerated iterator");
    // for (i, e) in ai32.iter().enumerate() {
    //     println!("a[{}]: {}", i, e);
    // }

    let mut m_ai32 = [1;10];
    // println!("mutable with i indices");
    // for i in 0..m_ai32.len() {
    //     println!("a[{}]: {}", i, m_ai32[i]);
    // }

    m_ai32[1] = 10;
    m_ai32[9] = 90;

    // println!("after mutated with enumerated iterator");
    // for (i, e) in m_ai32.iter().enumerate() {
    //     println!("a[{}]: {}", i, e);
    // }

    m_ai32.iter_mut().for_each(|x| *x +=  1);

    
    println!("after mutated iterator with enumerated iterator");
    for (i, e) in m_ai32.iter().enumerate() {
        println!("a[{}]: {}", i, e);
    }

    m_ai32
    .iter_mut()
    .for_each(|x| *x *= *x);
    
    m_ai32
    .iter()
    .enumerate()
    .for_each(|(i, e)| println!("a[{}]: {}", i, e));
}

pub fn primitives_in_vector() {
    let vi32 = vec![1, 2, 3, 4];

    println!("with indexed for");
    for i in 0..vi32.len() {
        println!("v[{}]: {}", i, vi32[i]);
    }
    
    println!("with enumerated iterator");
    vi32
    .iter()
    .enumerate()
    .for_each(|(i, x)| println!("v[{}]: {}", i, x));

    let mut m_vi32 = vec![1, 2, 3, 4];
 
    println!("mut with enumerated iterator");   
    m_vi32
    .iter()
    .enumerate()
    .for_each(|(i, x)| println!("v[{}]: {}", i, x));

    m_vi32[0] = 10;
    m_vi32.push(5);

    println!("mut with enumerated iterator after mutated.");
    m_vi32
    .iter()
    .enumerate()
    .for_each(|(i, x)| println!("a[{}]: {}, {}", i, x, (*x as i32).pow(i as u32)));

    m_vi32
    .iter_mut()
    .for_each(|x| *x = (*x as i32).pow(2));

    println!("mut with enumerated iterator after squared.");

    m_vi32
    .iter()
    .enumerate()
    .for_each(|(i, x)| println!("v[{}]: {}", i, x));

    square_vector(&mut m_vi32);
    println!("mut with enumerated iterator after squared in function.");

    m_vi32
    .iter()
    .enumerate()
    .for_each(|(i, x)| println!("v[{}]: {}", i, x));
}

fn square_vector(v: &mut Vec<i32>) {
    v
    .iter_mut()
    .for_each(|x| *x = (*x as i32).pow(2));
}


