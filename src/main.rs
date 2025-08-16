fn main() {
    let x: i8 =-5;
    let y : u32=10;
    let z: f32 = 3.14;
    print!("x: {}, y: {}, z: {}", x, y, z);

}

fn main(){
  let mut x: i32 = 10;

    for i in 0..1000 {
        x = x + 100;
    }
    print!("x: {}", x);
}

fn main() {
    let is_male: bool = true;
    let mut _is_above_18: bool = true;

    if is_male {
        println!("you are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && _is_above_18 {
        println!("You are a legal male");
    }
}

fn main() {
    let greeting = String::from("hello world");
    println!("Greeting: {}", greeting);
    match greeting.chars().nth(3) {
        Some(c) => println!("Character at index 3 is '{}'", c),
        None => println!("No character at that index!"),
    }
}

conditional and  loops

fn main() {
    let is_even = true;

    if is_even {
        println!("the number is even ");
    } else if is_even {
        println!("the number is odd");
    }
}


loops
fn main() {
    let sentence = String::from("my name is ritesh");
    let first_word = get_first_word(sentence);
    println!("First word: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        ans.push(c);
    }
    return ans;
}
 
functions

fn main(){
    let a: i32=10;
    let b: i32=20;
    let sum: i32 = do_sum(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}


