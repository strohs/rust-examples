use futures::core_reexport::fmt::Debug;

/// playing around with Vector slices

fn main() {
    let mut v1 = vec![60, 90, 100, 50, 70, 80];
    let mut v2 = vec![2, 4, 6, 8];
    merge(&mut v1);

    //dbg!(&v1);
}

fn merge<T: Ord + Debug>(slice: &mut[T]) {
    let mid = slice.len() / 2;
    let (l, r) = slice.split_at_mut(mid);

    let ss = l.get_mut(1..3).expect("left must have at least one element");
    println!("{:?}", &ss);
}

