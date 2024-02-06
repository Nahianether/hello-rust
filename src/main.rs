fn main(){
   let z = calculation(1, 2, 3);
println!("The value of z is: {z}");

let test_sentence = string_test();
println!("The value of test_sentence is: {test_sentence}");

let fib = fibonacci(10);
println!("The value of fib is: {fib}");
}

fn calculation(x: i8, y: i8, z: i8) -> i8 {
   return x + y * z / x - y;
}

fn string_test()-> String{
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";

   if greeting.is_empty() {
      return greeting.to_string();
   }

    let mut test_sentence = String::new();
    test_sentence.push_str(greeting);
    test_sentence.push_str(" from ");
    test_sentence.push_str(planet);
    test_sentence
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}