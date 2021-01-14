use std::iter::from_fn;

fn fib_sequence(s1:u128, s2: u128) -> impl Iterator<Item = u128> {
    let mut current = s1;
    let mut next = s2;

    from_fn(move || {
        next += current;
        current = next - current;
        Some(current)
    })
}

fn main() {
    fib_sequence(1,1).take(90).for_each(|n| println!("{}", n));
}