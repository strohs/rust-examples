

fn main() {
    //let ixs: Vec<usize> = (0..10).map(|i| i).collect();
    //dbg!(ixs);

    let nums = [1,2,3,8,8,8,9,10];

    let mut pos: usize = 0;
    if let Some(idx) = nums[..=5].iter().rposition(|n| *n != nums[4]) {
        pos = idx + 1
    } else {
        pos = 0
    }
    println!("pos: {}", pos);
}