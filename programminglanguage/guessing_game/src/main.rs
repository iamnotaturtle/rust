use std::cmp::Ordering;
use std::io;
extern crate rand;
use rand::Rng;

fn main() {
    rectangles();
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, rect: Rectangle) -> bool {
        return self.width > rect.width && self.height > rect.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

fn rectangles() {
    let rec = Rectangle {
        height: 30,
        width: 50,
    };
    let rec2 = Rectangle {
        height: 300,
        width: 50,
    };
    println!("The area of rectangle is {}", rec.area());
    println!("The rectangle is {:#?}", rec);
    println!("Can rec2 hold rec? {}", rec2.can_hold(rec));

    let sqr = Rectangle::square(4);
    println!("4x4 square is {:#?} and has an area of {}", sqr, sqr.area());
}

fn twelve_days_of_christmas() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let presents: [&str; 12] = [
        "partridge in a pear tree!",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings!",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Eight maids milkin'",
        "Nine ladies dancin'",
        "Ten lords a leapin'",
        "Eleven pipers pipin'",
        "Twelve drummers drummin'",
    ];
    let mut gifts: String = String::from("");

    days.iter().enumerate().for_each(|(i, day)| {
        println!("On the {} day of Christmas, my true love gave to me,", day);
        if i == 0 {
            gifts = format!("A {}", presents[i]);
        } else if i == 1 {
            gifts = format!("{}\nAnd a {}", presents[i], presents[0]);
        } else {
            gifts = format!("{}\n{}", presents[i], gifts);
        }
        println!("{}", gifts);
    })
}

fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _x in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    return c;
}

#[test]
fn test_fib() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(10), 89);
}

fn convert(t: f32, to_celcius: bool) -> f32 {
    return match to_celcius {
        true => (t - 32.0) / 1.8000,
        false => (t * 1.8000) + 32.0,
    };
}

#[test]
fn test_conversion() {
    assert_eq!(convert(32.0, true), 0.0);
    assert_eq!(convert(convert(32.0, true), false), 32.0);
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(0..100);
    println!("The number is {}", secret_number);

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
