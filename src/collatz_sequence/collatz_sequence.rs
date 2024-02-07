pub fn collatz_sequence(n: i32) -> i32{
    let mut n = n;
    let mut x = 0;
    while n > 1 {
        if n % 2 == 0 {
            x = n / 2;
        }else {
            x = 3 * n + 1;
        }
        n = n - 1;
    }
    x
}