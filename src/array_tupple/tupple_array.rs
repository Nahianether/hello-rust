pub fn tuple_array_test(){
    let mut t: [(i8, bool); 10] = [(7, true); 10];
    t[5] = (10, false);
    println!("t: {t:?}");
 }