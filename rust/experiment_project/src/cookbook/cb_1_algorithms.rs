use rand::Rng;

pub fn main() {
    random_experiments();
    vector_sort();
    custom_struct_sort();
}

////////RANDOM/////////

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

use rand::distributions::{Distribution, Standard};
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

pub fn random_experiments() {
    // test a few random calls
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {n1}");
    println!("Random u16: {n2}");
    println!("Random f32: {}", rng.gen::<f32>());
    println!("Random gen range 0..10: {}", rng.gen_range(0..10));
    println!("Random gan range 0.0..10.0: {}", rng.gen_range(0.0..10.0));
    // uniform distribution
    use rand::distributions::Uniform;
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
    // Normal distribution
    // another module - rand_distr
    {
        use rand_distr::Normal;
        let normal = Normal::new(2.0, 3.0).unwrap();
        let v = normal.sample(&mut rng);
        println!("{v} is from a N(2, 9) distribution");
    }
    // custom types
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    println!("Rand tuple (i32, bool, f64): {:?}", rand_tuple);
    let rand_point: Point = rng.gen();
    println!("Rand point: {:?}", rand_point);
    {
        // random string
        use rand::distributions::Alphanumeric;
        use rand::thread_rng;
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        println!("Random string: {rand_string}");
        // random password
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;
        let mut rng = rand::thread_rng();

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        println!("Random password: {:?}", password);
    }
}

fn vector_sort() {
    {
        let mut vec = vec![1, 4, 5, 10, 2, 15];
        print!("Vector: {:?}", vec);
        vec.sort();
        println!(", sorted: {:?}", vec);
    }
    {
        let mut vec = vec![1.7, 4.0, 5.5, 10.0, 10.1];
        print!("Vector: {:?}", vec);
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(", sorted: {:?}", vec);
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn custom_struct_sort() {
    let mut people = vec![
        Person::new("zoe".to_string(), 32),
        Person::new("Al".to_string(), 60),
        Person::new("Pam".to_string(), 1),
    ];

    println!("Raw vector: {:?}", people);
    people.sort();
    println!("Sorted vector: {:?}", people);
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("Sorted vector by age: {:?}", people);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main_rng() {
        main();
    }
}
