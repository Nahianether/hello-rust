pub fn test_while_loop(x: i32)-> String{
    let mut x = x;
    while x >= 10 {
        x = x / 2;
    }
    return format!("Final x: {x}");
 }