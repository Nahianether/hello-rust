// fn main(){
//    let z = calculation(1, 2, 3);
// println!("The value of z is: {z}");
// }

// fn calculation(x: i32, y: i32, z: i32) -> i32 {
//    return x + y * z / x - y;
// }


fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    //println!("{:?}", &sentence[12..13]);
}