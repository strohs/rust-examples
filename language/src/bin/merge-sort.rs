/// playing around with Vector slices

fn main() {
    let mut v1 = vec![100, 60, 90, 70, 80, 50];
    mergesort(&mut v1[..]);

    dbg!(&v1);
}

// move the first element of slice into its proper sorted position within the slice
// the sort exits the first time the element is NOT swapped
fn bubble<T: Ord>(slice: &mut[T]) {
    for i in 0..(slice.len() - 1) {
        if slice[i] > slice[i + 1] {
            slice.swap(i, i + 1);
        } else {
            break
        }
    }
}

fn mergesort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        _ => {},
    }

    // split the slice at the midpoint into a left,right halves
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at_mut(mid);

    mergesort(left);
    mergesort(right);

    //println!("{:?} {:?}", &left, &right);

    // merge left and right together, into sorted order
    for l in 0..left.len() {
        if right[0] < left[l] {
            std::mem::swap(&mut left[l], &mut right[0]);
            let i = match right[1..].binary_search(&right[0]) {
                Ok(i) => i,
                Err(i) => i,
            };
            let i = i + 1;
            right[..i].rotate_left(1);
        }
    }
    //println!("after  {:?} {:?}", &left, &right);

}
