struct Fib {
    c: u32,
    n: u32
}

impl Fib {
    fn new() -> Fib {
        Fib { c: 0, n: 1}
    }

    /// returns the nth number of the Fibonacci sequence
    fn nth(index: usize) -> u32 {
        if index == 0 || index == 1 {return index as u32;}
        let f = Fib::new();
        f.take(index).last().unwrap()
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.n)
    }
}

fn main() {
    let f = Fib::new();
    dbg!( Fib::nth(25) );
}