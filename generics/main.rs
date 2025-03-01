fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = get_largest(number_list);
    println!("thelargets numebr is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let mut largest = get_largest(char_list);
    println!("the largest char is {}", largest);
}

// Bad Approach here

// fn get_largest(list: Vec<i32>) -> i32 {
//     let mut largest = list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// fn get_largest_char(list: Vec<char>) -> char {
//     let mut largest = list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// Better Approach
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    //using traits
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
