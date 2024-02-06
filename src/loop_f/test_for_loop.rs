pub fn test_for_loop(x: i32)-> String{
    let mut x = x;
    loop {
       x = x + 1;
       if x == 10 {
          return format!("loop finished, now x is: {x}");
       }
    }
 }