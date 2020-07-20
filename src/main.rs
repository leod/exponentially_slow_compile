#![type_length_limit = "320000000000"]

fn nest<F: Fn(usize) -> usize>(f: F) -> impl Fn(usize) -> usize {
    move |t| f(t)
}

fn main() {
    println!("Hello, world!");

    let f = |_| 42;

    // 1
    let f = nest(f);

    // 2
    let f = nest(f);

    // 4
    let f = nest(f);
    let f = nest(f);

    // 8
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);

    // 16
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);

    // 24
    //    Finished dev [unoptimized + debuginfo] target(s) in 55.79s
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);

    // 32
    //    Takes a long time, I got tired of waiting
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);
    let f = nest(f);

    println!("{}", f(42));
}
