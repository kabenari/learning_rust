// fn main() {
//     let x = helper(9);
//     println!("{}", x);
// }

// fn helper(x: i32) -> i32 {
//     return x + 1;
// }

fn main() {
    let mut counter = 0;

    let result = helper(counter);

    println!("result is : {result}");
}

fn helper(mut counter: i32) -> i32 {
    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    return counter;
}
