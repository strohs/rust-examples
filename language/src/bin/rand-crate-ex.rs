use rand::{thread_rng};
use rand::seq::{SliceRandom};

fn main() {
    let mut rng = thread_rng();
    // We can also interact with iterators and slices:
    //let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    //println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);
}