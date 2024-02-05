fn main(){
   let z = calculation(1, 2, 3);
println!("The value of z is: {z}");

let test_sentence = string_test();
println!("The value of test_sentence is: {test_sentence}");
}

fn calculation(x: i32, y: i32, z: i32) -> i32 {
   return x + y * z / x - y;
}

fn string_test()-> String{
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";

    let mut test_sentence = String::new();
    test_sentence.push_str(greeting);
    test_sentence.push_str(" from ");
    test_sentence.push_str(planet);
    return test_sentence;
}