pub fn main() {
    matrix::basics();
    matrix_vect::main();
    vect_norm::main();
    matrix_invert::main();
    trig::len_triangle();
    trig::trig_test();
    trig::earth_dist();
    complex::create();
    complex::add();
    complex::math();
    stat::central_tendency::main();
    stat::median::main();
    stat::mode::main();
    stat::std_dev::main();
    big_int::main();
}

mod matrix {
    use ndarray::arr2;

    pub fn basics() {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

        let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

        let sum = &a + &b;

        println!("Adding matrices");
        println!("{}", a);
        println!("+");
        println!("{}", b);
        println!("=");
        println!("{}", sum);

        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let b = arr2(&[[6, 3], [5, 2], [4, 1]]);
        println!("Multiplying matrices");
        println!("{}", a);
        println!("*");
        println!("{}", b);
        println!("=");
        println!("{}", a.dot(&b));
    }
}

mod matrix_vect {
    use ndarray::{arr1, arr2, Array1};

    pub fn main() {
        println!("Multilying matrix by vector");
        let scalar = 4;

        let vector = arr1(&[1, 2, 3]);

        let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

        let new_vector: Array1<_> = scalar * vector;
        println!("{}", new_vector);

        let new_matrix = matrix.dot(&new_vector);
        println!("{}", new_matrix);
    }
}

// mod vect_comp {
//
//     use approx::assert_abs_diff_eq;
//     use ndarray::Array;
//
//     fn main() {
//         let a = Array::from(vec![1., 2., 3., 4., 5.]);
//         let b = Array::from(vec![5., 4., 3., 2., 1.]);
//         let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
//         let mut d = Array::from(vec![5., 4., 3., 2., 1.]);
//
//         let z = a + b;
//         let w = &c + &d;
//
//         assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));
//
//         println!("c = {}", c);
//         c[0] = 10.;
//         d[1] = 10.;
//
//         assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));
//     }
// }

mod vect_norm {

    use ndarray::{array, Array1, ArrayView1};

    fn l1_norm(x: ArrayView1<f64>) -> f64 {
        x.fold(0., |acc, elem| acc + elem.abs())
    }

    fn l2_norm(x: ArrayView1<f64>) -> f64 {
        x.dot(&x).sqrt()
    }

    fn normalize(mut x: Array1<f64>) -> Array1<f64> {
        let norm = l2_norm(x.view());
        x.mapv_inplace(|e| e / norm);
        x
    }

    pub fn main() {
        let x = array![1., 2., 3., 4., 5.];
        println!("||x||_2 = {}", l2_norm(x.view()));
        println!("||x||_1 = {}", l1_norm(x.view()));
        println!("Normalizing x yields {:?}", normalize(x));
    }
}

mod matrix_invert {

    use nalgebra::Matrix3;

    pub fn main() {
        let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
        println!("m1 = {}", m1);
        match m1.try_inverse() {
            Some(inv) => {
                println!("The inverse of m1 is: {}", inv);
            }
            None => {
                println!("m1 is not invertible!");
            }
        }
    }
}

// mod matrix_serialize {
//
//     use nalgebra;
//     use serde_json;
//
//     use nalgebra::DMatrix;
//
//     pub fn main() -> Result<(), std::io::Error> {
//         let row_slice: Vec<i32> = (1..5001).collect();
//         let matrix = DMatrix::from_row_slice(50, 100, &row_slice);
//
//         // serialize matrix
//         let serialized_matrix = serde_json::to_string(&matrix)?;
//
//         // deserialize matrix
//         let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;
//
//         // verify that `deserialized_matrix` is equal to `matrix`
//         assert!(deserialized_matrix == matrix);
//
//         Ok(())
//     }
// }

mod trig {
    pub fn len_triangle() {
        let angle: f64 = 2.0;
        let side_length = 80.0;

        let hypotenuse = side_length / angle.sin();

        println!("Hypotenuse: {}", hypotenuse);
    }

    pub fn trig_test() {
        let x: f64 = 6.0;

        let a = x.tan();
        let b = x.sin() / x.cos();

        assert_eq!(a, b);
    }

    pub fn earth_dist() {
        let earth_radius_kilometer = 6371.0_f64;
        let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);
        let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

        let paris_latitude = paris_latitude_degrees.to_radians();
        let london_latitude = london_latitude_degrees.to_radians();

        let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
        let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

        let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
            + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
        let central_angle = 2.0 * central_angle_inner.sqrt().asin();

        let distance = earth_radius_kilometer * central_angle;

        println!(
            "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
            distance
        );
    }
}

mod complex {

    pub fn create() {
        let complex_integer = num::complex::Complex::new(10, 20);
        let complex_float = num::complex::Complex::new(10.1, 20.1);

        println!("Complex integer: {}", complex_integer);
        println!("Complex float: {}", complex_float);
    }

    pub fn add() {
        let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
        let complex_num2 = num::complex::Complex::new(3.1, -4.2);

        let sum = complex_num1 + complex_num2;

        println!("Sum: {}", sum);
    }

    use num::complex::Complex;
    use std::f64::consts::PI;

    pub fn math() {
        let x = Complex::new(0.0, 2.0 * PI);

        println!("e^(2i * pi) = {}", x.exp()); // =~1
    }
}

mod stat {
    pub mod central_tendency {
        pub fn main() {
            let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

            let sum = data.iter().sum::<i32>() as f32;
            let count = data.len();

            let mean = match count {
                positive if positive > 0 => Some(sum / count as f32),
                _ => None,
            };

            println!("Mean of the data is {:?}", mean);
        }
    }

    pub mod median {

        use std::cmp::Ordering;

        fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
            match data.len() {
                0 => None,
                _ => {
                    let (pivot_slice, tail) = data.split_at(1);
                    let pivot = pivot_slice[0];
                    let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                        {
                            let (ref mut left, ref mut right) = &mut splits;
                            if next < &pivot {
                                left.push(*next);
                            } else {
                                right.push(*next);
                            }
                        }
                        splits
                    });

                    Some((left, pivot, right))
                }
            }
        }

        fn select(data: &[i32], k: usize) -> Option<i32> {
            let part = partition(data);

            match part {
                None => None,
                Some((left, pivot, right)) => {
                    let pivot_idx = left.len();

                    match pivot_idx.cmp(&k) {
                        Ordering::Equal => Some(pivot),
                        Ordering::Greater => select(&left, k),
                        Ordering::Less => select(&right, k - (pivot_idx + 1)),
                    }
                }
            }
        }

        fn median(data: &[i32]) -> Option<f32> {
            let size = data.len();

            match size {
                even if even % 2 == 0 => {
                    let fst_med = select(data, (even / 2) - 1);
                    let snd_med = select(data, even / 2);

                    match (fst_med, snd_med) {
                        (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                        _ => None,
                    }
                }
                odd => select(data, odd / 2).map(|x| x as f32),
            }
        }

        pub fn main() {
            let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

            let part = partition(&data);
            println!("Partition is {:?}", part);

            let sel = select(&data, 5);
            println!("Selection at ordered index {} is {:?}", 5, sel);

            let med = median(&data);
            println!("Median is {:?}", med);
        }
    }

    pub mod mode {

        use std::collections::HashMap;

        pub fn main() {
            let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

            let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
                *freqs.entry(value).or_insert(0) += 1;
                freqs
            });

            let mode = frequencies
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(value, _)| *value);

            println!("Mode of the data is {:?}", mode);
        }
    }

    pub mod std_dev {
        fn mean(data: &[i32]) -> Option<f32> {
            let sum = data.iter().sum::<i32>() as f32;
            let count = data.len();

            match count {
                positive if positive > 0 => Some(sum / count as f32),
                _ => None,
            }
        }

        fn std_deviation(data: &[i32]) -> Option<f32> {
            match (mean(data), data.len()) {
                (Some(data_mean), count) if count > 0 => {
                    let variance = data
                        .iter()
                        .map(|value| {
                            let diff = data_mean - (*value as f32);

                            diff * diff
                        })
                        .sum::<f32>()
                        / count as f32;

                    Some(variance.sqrt())
                }
                _ => None,
            }
        }

        pub fn main() {
            let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

            let data_mean = mean(&data);
            println!("Mean is {:?}", data_mean);

            let data_std_deviation = std_deviation(&data);
            println!("Standard deviation is {:?}", data_std_deviation);

            let zscore = match (data_mean, data_std_deviation) {
                (Some(mean), Some(std_deviation)) => {
                    let diff = data[4] as f32 - mean;

                    Some(diff / std_deviation)
                }
                _ => None,
            };
            println!(
                "Z-score of data at index 4 (with value {}) is {:?}",
                data[4], zscore
            );
        }
    }
}

mod big_int {

    use num::bigint::{BigInt, ToBigInt};

    fn factorial(x: i32) -> BigInt {
        if let Some(mut factorial) = 1.to_bigint() {
            for i in 1..=x {
                // factorial = factorial * i;
                factorial *= i;
            }
            factorial
        } else {
            panic!("Failed to calculate factorial!");
        }
    }

    pub fn main() {
        println!("{}! equals {}", 100, factorial(100));
    }
}
