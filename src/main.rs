// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
fn main() {
    println!("-----------");
    let xs: [i32; 4] = [1, 2, 3, 4];

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("element {:?}", xval),
            None => println!("element not found in index {}", i)
        }
    }

    println!("-----------");
}
