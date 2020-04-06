fn main() {
    //let ixs: Vec<usize> = (0..10).map(|i| i).collect();
    //dbg!(ixs);

    let nums = [1,2,3,5,8,8,9,10];
    let cmi: usize = 4;
    let mut pos: usize = 0;
    if let Some(idx) = nums[..=cmi].iter().rposition(|n| *n != nums[cmi]) {
        pos = idx + 1
    } else {
        pos = 0
    }
    println!("pos: {}", pos);
}