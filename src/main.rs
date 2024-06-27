fn main() {
    let x = another_function(57);
    let a = (1, 2, "a");
    println!("{}", x);
    //loop_function();
    while_function();
    for_function();
    let a = reverse((1, false));
    print!("{:?}", a);
    let i = (1..4)
    println!("{}", i[2]);
}
//
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}
 
fn another_function(i: i32) -> i32 {
    /* 
    if i < 5 {
         i + 5
    }
    else {
         i + 6
    }*/
    match i {
        n if n < 5 => i + 5,
        _ => i + 6
        
        
    }
}

fn for_function() {
    let mut b = [0];
    let a = [1, 2, 3, 4, 5];
    let _c = [0; 5];
    b[0] = 10;

    for element in a.iter() {
        println!("{}", element)
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn while_function() {
    let mut i = 1;

    while i < 5 {
        println!("{}", i);
        i += 1;
    }
}

fn loop_function() {
    loop {
        println!("dllm");
    }
}