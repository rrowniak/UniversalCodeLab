use std::path::PathBuf;
use std::sync::mpsc::channel;

#[allow(dead_code)]
pub fn main() {
    println!("Concurrency demo");
    short_lived_threads();
    println!("------------------------");
    parallel_pipeline();
    println!("------------------------");
    pass_data_two_threads();
    println!("------------------------");
    global_mutable_state();
    println!("------------------------");
    compute_julia();
    println!("------------------------");
    rayon_experiments();
    println!("------------------------");
}

#[allow(dead_code)]
fn short_lived_threads() {
    let arr = [1, 25, -4, 10, 26];
    let max = find_max(&arr);
    assert_eq!(max, Some(26));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().copied().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap();
        let max_r = thread_r.join().unwrap();

        Some(max_l.max(max_r))
    })
    .unwrap()
    .unwrap()
}

fn parallel_pipeline() {
    use crossbeam_channel::bounded;
    use std::thread;
    use std::time::Duration;

    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(2);

    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source send {}", i);
            }
            // close the channel
            // necessary to exit for-loop in the worker
            drop(snd1);
        });

        // processing
        for _ in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());

            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }

        drop(snd2);

        // sink
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

fn pass_data_two_threads() {
    use crossbeam_channel::unbounded;
    use std::{thread, time};

    let (snd, rcv) = unbounded();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}

fn global_mutable_state() {
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
    }

    fn insert(fruit: &str) {
        let mut db = FRUIT
            .lock()
            .map_err(|_| "Failed to acquire MutexGuard")
            .unwrap();
        db.push(fruit.to_string());
    }

    insert("apple");
    insert("orange");
    insert("peach");

    {
        let db = FRUIT
            .lock()
            .map_err(|_| "Failed to acquire MutexGuard")
            .unwrap();
        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape");
}

use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use threadpool::ThreadPool;
use walkdir::WalkDir;

pub fn sha256_files(path: &PathBuf) {
    println!("Calculating sha256 for {:?} directory...", path);
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir())
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).expect("Could not send data");
        });
    }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t.unwrap();
        println!("{:?} {:?}", sha, path);
    }
}

fn compute_julia() {
    use image::{ImageBuffer, Rgb};
    use num::complex::Complex;

    let (width, height) = (1920, 1080);

    fn julia(c: Complex<f32>, x: u32, y: u32, w: u32, h: u32, max_iteration: u32) -> u32 {
        const R: f32 = 3.0;
        let mut zy = (y as f32 / (h as f32)) * R - R / 2.0;
        let mut zx = (x as f32 / (w as f32)) * R - R / 2.0;

        let mut iteration = 0;
        while zx * zx + zy * zy < R * R && iteration < max_iteration {
            let xtemp = zx * zx - zy * zy;
            zy = 2.0 * zx * zy + c.im;
            zx = xtemp + c.re;
            iteration += 1;
        }
        iteration
    }

    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                // let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                let r = iterations - i;
                let r = r as f32 / iterations as f32;
                let r = 255.0 - r * 255.0;
                let pixel = Rgb([r as u8, 0, 0]);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png");
}

fn compute_digest(filepath: PathBuf) -> Result<(Digest, PathBuf), Error> {
    let mut buf_reader = BufReader::new(File::open(filepath.clone())?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

struct Person {
    age: u32,
}

fn rayon_experiments() {
    use rayon::prelude::*;

    println!("Mutate the elements of an array in parellel");
    let mut arr = [0, 7, 9, 11, 13, 18];
    println!("Orig array: {:?}", arr);
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("Array: {:?}", arr);

    println!("Test in parallel if any or all elements of a collection match a given predicate");
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));

    println!("Search items using given predicate in parallel");
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));

    println!("Sort a vector in parallel");
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        let v = (0..5).map(|_| rng.sample(&Alphanumeric)).collect();
        *p = String::from_utf8(v).unwrap();
    });
    vec.par_sort_unstable();
    println!(
        "First vec elements: {:?}, last vec elements: {:?}",
        &vec[..10],
        &vec[vec.len() - 10..]
    );

    println!("Map-reduce in parallel");
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}

use std::fs::create_dir_all;
use std::path::Path;

use glob::{glob_with, MatchOptions};
use image::imageops::FilterType;
use image::ImageError;
use rayon::prelude::*;

pub fn generate_thumbnails_parallel(path: &PathBuf) {
    let options: MatchOptions = Default::default();
    let p: String = path.to_str().unwrap().to_string() + "/*.jpg";
    let files: Vec<_> = glob_with(&p, options)
        .unwrap()
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        panic!("No .jpg files found in current directory");
    }

    let thumb_dir = path.to_str().unwrap().to_string() + "/thumbnails";
    create_dir_all(&thumb_dir).unwrap();

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|name| {
            make_thumbnail(name.file_name().unwrap(), path, &thumb_dir, 300)
            // .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x));
    // .for_each(|x| println!("{}", x.display_chain()));

    println!(
        "{} thumbnails saved successfully",
        files.len() - image_failures.len()
    );
}

fn make_thumbnail<PA, PB, PC>(
    name: PC,
    original: PA,
    thumb_dir: PB,
    longest_edge: u32,
) -> Result<(), ImageError>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
    PC: AsRef<Path>,
{
    println!(
        "name: {:?}, original: {:?}, thumb_dir: {:?}",
        name.as_ref(),
        original.as_ref(),
        thumb_dir.as_ref()
    );
    let img = image::open(original.as_ref().join(&name))?;
    let file_path = thumb_dir.as_ref().join(name);
    println!("Saving at {:?}...", file_path);

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_short_lived_threads() {
        short_lived_threads();
    }
}
