pub fn scope_and_shadowing(a: i32) -> i32{
    println!("start: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("scope and shadow function result: {a}");
    return a;
}