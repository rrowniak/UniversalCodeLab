```rust
#[derive(Default, Debug)]
struct S {
    every: i32,
    field: i32,
    except: i32,
}

impl S {
    fn new() -> Self {
        S {
            ..Default::default()
        }
    }
}

fn main() {
    let s0 = S::new();
    println!("s0: {s0:?}");
    let s1 = S {
        every: 1,
        field: 2,
        except: 5
    };
    println!("s1: {s1:?}");
    // struct update syntax
    let s2 = S {
        every: 1,
        field: 2,
        ..s1
    };
    println!("s2: {s2:?}");
    // assign
    let s3 = s2;
    println!("s3: {s3:?}");
    // struct update syntax - default
    // #[derive(Default)] needed
    let s4 = S {
        every: 3,
        field: 4,
        ..Default::default()
    };
    println!("s4: {s4:?}");
}

```
Results:
```
s0: S { every: 0, field: 0, except: 0 }
s1: S { every: 1, field: 2, except: 5 }
s2: S { every: 1, field: 2, except: 5 }
s3: S { every: 1, field: 2, except: 5 }
s4: S { every: 3, field: 4, except: 0 }
```
