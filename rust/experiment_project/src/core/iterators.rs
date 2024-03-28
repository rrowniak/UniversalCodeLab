pub fn main() {
    basic_usage();
    borrow_ref();
    move_ref();
    borrow_mut_ref();
    ranges();
    it_transforms_iteration_process();
    it_transforms();
    it_filter();
}

fn basic_usage() {
    let colors = vec!["Red", "Yellow", "Green"];

    let mut colors_iterator = colors.iter();

    assert_eq!(colors_iterator.next(), Some(&"Red"));
    assert_eq!(colors_iterator.next(), Some(&"Yellow"));
    assert_eq!(colors_iterator.next(), Some(&"Green"));
    assert_eq!(colors_iterator.next(), None);
}

fn borrow_ref() {
    let colors = vec!["Red", "Yellow", "Green"];
    for c in colors.iter() {
        println!("Color: {}", c);
    }
    println!("We can still use colors: {:?}", colors);
}

fn move_ref() {
    let colors = vec!["Red", "Yellow", "Green"];
    for c in colors.into_iter() {
        println!("Color: {}", c);
    }
    // we cannot use colors anymore
    // println!("Will not compile: {:?}", colors);
}

fn borrow_mut_ref() {
    let mut colors = vec!["Red", "Yellow", "Green"];
    for c in colors.iter_mut() {
        println!("Original: {}", c);
        // modify
        *c = "Only black!";
    }

    println!("Modified collection: {:?}", colors);
}

fn ranges() {
    println!("Looping through a range 1..6: ");
    for i in 1..6 {
        print!("{}, ", i);
    }
    println!();
    println!("Looping through a range 1..=6: ");
    let r1 = 1..=6;
    for i in r1 {
        print!("{}, ", i);
    }
    println!();
}

fn it_transforms_iteration_process() {
    let ints = 0..=10;
    println!("Original collection:");
    for i in ints.clone() {
        print!("{}, ", i);
    }
    println!();

    println!("take(4):");
    for i in ints.take(4) {
        print!("{}, ", i);
    }
    println!();

    println!("skip(4):");
    for i in (0..=10).skip(4) {
        print!("{}, ", i);
    }
    println!();

    println!("step_by(2):");
    for i in (0..=10).step_by(2) {
        print!("{}, ", i);
    }
    println!();

    println!("chain(25..30):");
    for i in (0..=10).chain(25..30) {
        print!("{}, ", i);
    }
    println!();

    println!("cycle() - print until sum > 100");
    let mut sum = 0;
    for i in (0..=10).cycle() {
        sum += i;
        if sum > 100 {
            break;
        }
        print!("{}, ", i);
    }
    println!();

    println!("rev()");
    for i in (0..=10).rev() {
        print!("{}, ", i);
    }
    println!();
}

fn it_transforms() {
    let up = 10;
    println!("map(|x| x*x)");
    for i in (0..=up).map(|x| x * x) {
        print!("{}, ", i);
    }
    println!();

    println!("map(|x| format!(\"i={{}}\", x))");
    for i in (0..=up).map(|x| format!("i={}", x)) {
        print!("{}, ", i);
    }
    println!();

    println!("rev().enumerate()");
    for (i, elem) in (0..=up).rev().enumerate() {
        print!("{} at {}, ", elem, i);
    }
    println!();

    println!("zip(50..55)");
    for (i, j) in (0..=up).zip(50..55) {
        print!("{} and {}, ", i, j);
    }
    println!();
}

fn it_filter() {
    println!("filter(|x| x % 3 == 0)");
    for i in (0..=10).filter(|x| x % 3 == 0) {
        print!("{}, ", i);
    }
    println!();

    println!("take_while(|x| *x < 3)");
    for i in (0..=10).take_while(|x| *x < 3) {
        print!("{}, ", i);
    }
    println!();

    println!("skip_while(|x| *x < 3)");
    for i in (0..=10).skip_while(|x| *x < 3) {
        print!("{}, ", i);
    }
    println!();
}
