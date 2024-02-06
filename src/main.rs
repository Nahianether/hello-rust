pub mod calculation;
use crate::calculation::calculation;

pub mod loop_f;
use crate::loop_f::test_for_loop::test_for_loop;
use crate::loop_f::test_while_loop::test_while_loop;

pub mod fibonacci;
use crate::fibonacci::fibonacci;

fn main(){
   let z = calculation(1, 2, 3);
println!("The value of z is: {z}");

let test_sentence = string_test();
println!("The value of test_sentence is: {test_sentence}");

let fib = fibonacci(10);
println!("The value of fib is: {fib}");

let check = check_if_else(10);
println!("The value of check is: {check}");

let test_loop = test_for_loop(0);
println!("The value of test_loop is: {test_loop}");

let test_while_loop = test_while_loop(200);
println!("The value of while_loop is: {test_while_loop}");
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

fn check_if_else(x: i64) -> String{
   if x < 10{
      return "less than 10".to_string();
   }else if x > 10{
      return "greater than 10".to_string();
   }else {
      return "equal to 10".to_string();
   }
}