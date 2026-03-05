fn main() {
    let n: u64 = 27;
    let iterations: u64 = foo(n);
    println!("{iterations}");
}

fn foo(mut n: u64) -> u64 {
    let mut i = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        i += 1;
    }
    i
}