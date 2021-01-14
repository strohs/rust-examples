// example of trying to write a custom "safe-get" trait on Vec

trait SafeGetter<T> {
    fn safe_get(&self, index: i32) -> Option<&T>;
}

impl <T> SafeGetter<T> for Vec<T> {
    fn safe_get(&self, index: i32) -> Option<&T> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}

fn main() {
    let v = vec![1,2,3,4,5];
    for i in -2..8 {
        dbg!(&v.safe_get(i));
    }
}