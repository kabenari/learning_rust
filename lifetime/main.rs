fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("World");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

//'a is the generic lifetime parameter
//'a is the lifetime of the parameter string1
//&i32 a reference to an i32
//&'a i32 a reference with an explicit lifetime

//1.Each parameter that is a refrece gets its own lifetime parameter

//2.if there is exactly one inpit lifetime parameter , that lifetime is assigned to all the output lifetime parameters

//3.If there are multiple inpuit lifetime parameters , but one of them is a &self or &mut self the lifeimte of self is assigned to all the output lifetime parameters

fn longest<'a>(a1: &'a str, a2: &'a str) -> &'a str {
    if a1.len() > a2.len() {
        a1
    } else {
        a2
    }
}


