// fn main() {
//     let x: i8 =-5;
//     let y : u32=10;
//     let z: f32 = 3.14;
//     print!("x: {}, y: {}, z: {}", x, y, z);

// }

// fn main(){
//   let mut x: i32 = 10;

//     for i in 0..1000 {
//         x = x + 100;
//     }
//     print!("x: {}", x);
// }

// fn main() {
//     let is_male: bool = true;
//     let mut _is_above_18: bool = true;

//     if is_male {
//         println!("you are a male");
//     } else {
//         println!("You are not a male");
//     }

//     if is_male && _is_above_18 {
//         println!("You are a legal male");
//     }
// }

// fn main() {
//     let greeting = String::from("hello world");
//     println!("Greeting: {}", greeting);
//     match greeting.chars().nth(3) {
//         Some(c) => println!("Character at index 3 is '{}'", c),
//         None => println!("No character at that index!"),
//     }
// }

// conditional and  loops

// fn main() {
//     let is_even = true;

//     if is_even {
//         println!("the number is even ");
//     } else if is_even {
//         println!("the number is odd");
//     }
// }


// loops
// fn main() {
//     let sentence = String::from("my name is ritesh");
//     let first_word = get_first_word(sentence);
//     println!("First word: {}", first_word);
// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for c in sentence.chars() {
//         if c == ' ' {
//             break;
//         }
//         ans.push(c);
//     }
//     return ans;
// }
 
// functions

// fn main(){
//     let a: i32=10;
//     let b: i32=20;
//     let sum: i32 = do_sum(a, b);
//     println!("Sum of {} and {} is {}", a, b, sum);
// }

// fn do_sum(a: i32, b: i32) -> i32 {
// 	return a + b;
// }


// Mutablity 
// fn main(){
//     let mut _x: i32 =1;
//     _x=2;
//     println!("x: {}", _x);
// }

// fn main() {
//     stack_fn();   // Call the function that uses stack memory
//     heap_fn();    // Call the function that uses heap memory
//     update_string();  // Call the function that changes size of variable at runtime
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);

//     // Append some text to the string
//     s.push_str(" and some additional text");
//     println!("After update: {}", s);
// 

fn main(){
 let _x: i32 = 10;
 println!("Value of x: {}", _x);
}


