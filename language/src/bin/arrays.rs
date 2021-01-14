

fn x2(n: &i32) -> i32 {
    n * 2
}

fn main() {
    const s: usize = 5;
    let mut arr: [u32; s] = [0; s];
    arr[0] = 11;
    dbg!(arr);

    // 2d arrays
    let mut arr2: [[u32; 3]; 3] = [[0; 3]; 3];
    arr2[2][2] = 4444;
    dbg!(arr2);

    let mut a = 23;
    let b = a;
    a = 46;
    dbg!(&a);
    dbg!(&b);

    let v = vec![1,2,3,4,5,6];
    let v2: Vec<i32> = v.iter().map(x2).collect();
    dbg!(v2);
}