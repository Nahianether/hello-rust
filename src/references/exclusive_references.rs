pub fn exclusive_references_fn(){
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("exclusive references: {point:?}");
}