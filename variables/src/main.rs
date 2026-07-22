fn main() {
    let tup: (i64, i32, i8) = (12,4,8); // list type of any combination of data types
    let (a,b,c) = tup;
    println!("our tuple values are: {}, {}, {}",a,tup.1,c);

    let array:[i32; 5] = [1,2,3,4,5]; // list type of 1 data type

    let x = 6;
    let x = x + 1;
    {
        let x = x * 2;
        println!("local scope x = {x}")
    }
    println!("x = {x}")
}
