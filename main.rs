fn sum_owned(v: Vec<i32>) -> i32 {
    let mut total = 0;
    for n in &v { total += *n; } 
    total
}

fn main() {
    let v = vec![1, 2, 3, 4, 5]; 
    let s = sum_owned(v);
    println!("Rust sum = {}", s);

    let r: &i32;
    {
        let x = 10;
        r = &x;                
    }                         
    println!("{}", r);
}
