fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}


fn main() {
    println!("{}", fib(9));
}
