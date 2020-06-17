/// playing around with Vector slices

fn main() {
    let mut v1 = vec![3,2,1,6,5,4,0,9,8,7];
    slice_sort(&mut v1[..]);

    //dbg!(&v1);

    let rep = vec![9,9,9];
    let v1_slice = &mut v1[2..5];
    for (i,n) in rep.iter().enumerate() {
        v1_slice[i] = *n;
    }

    dbg!(&v1);
}

fn slice_sort(v: &mut [i32]) {
    println!("slice size is {}", &v.len());
    println!("slice index 1 is {}", &v[1]);
    v.sort();
}