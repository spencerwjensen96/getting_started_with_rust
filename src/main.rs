use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn person() {
    let person: (&str, u32, u64) = ("Spencer", 24, 178);

    let (x,y,z) = person;

    println!("Name: {}", x);

    println!("Age: {}", y);

    println!("Height: {}", z);
}

struct Person {
    firstName: &str,
    lastName: &str,
    age: u32,
    height: u64,
}

impl toString for Person {
    fn toString(&self) {
        println!("\nstruct toString() =>");
        println!("\nFirst Name: {}", self.firstName);
        println!("\nLast Name: {}", self.lastName);
        println!("\nAge: {}", self.age);
        println!("\nHeight: {}", self.height);
    }
}

impl Person {
    fn new(firstName: String, lastName: String, age: u32, height: u64) -> Self {
        return Person {
            firstName: firstName,
            lastName: lastName,
            age: age,
            height: height,
        };
    }
}

fn number_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {

    let mut spencer = Person::new{
        firstName: "spencer",
        lastName: "jensen",
        age: 24,
        height: 175,
    };
    spencer.toString();
    

    person();
    number_game();
        
}
