
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; Won't compile
    // println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut y = 3;
    println!("The value of y is: {}", y);
    y = 4;
    println!("The value of y is: {}", y);

    const Z_CONSTANT: u32 = 20_000;
    println!("The value of z is: {}", Z_CONSTANT);

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let t = true;
    println!("true: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("false: {}", f);

    let standard_z = 'z';
    println!("standard_z: {}", standard_z);
    let fancy_z = 'ℤ';
    println!("fancy_z: {}", fancy_z);
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (_a, b, _c) = tup;  // underscore in front of unused variables
    println!("The value of b is: {}", b);
    let five_hun =  tup.0;
    println!("five_hun: {}", five_hun);
    let one = tup.2;
    println!("one: {}", one);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First Element: {}", arr[0]);
    let same_val_arr = [3; 5];
    println!("Same val array: {}, {}", same_val_arr[0], same_val_arr[2]);
}
