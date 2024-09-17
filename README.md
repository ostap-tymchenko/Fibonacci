### Main segment

```Rust
fn fib(position: i32) -> i128 {
    let mut first = 1;
    let mut second = 1;
    let mut new = 0;
    for iter in 2..position {
        new = second + first;
        println!("iter:{iter}, fib: {new}");
        second = first;
        first = new;
    }
    new
}
```
