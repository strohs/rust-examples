/// playing around with Vector slices

fn main() {
    let mut v1 = vec![3,2,1,6,5,4,0,9,8,7];
    slice_sort(&mut v1[3..]);

    dbg!(v1);
}

fn slice_sort(v: &mut [i32]) {
    v.sort();
}