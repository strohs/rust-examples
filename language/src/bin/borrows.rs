/// `mut foo: T` means you have a variable called `foo` that is a `T`.
/// You are allowed to change what the variable **refers to**
/// ```
/// let mut val1 = 2;
/// val1 = 3; // OK
///
/// let val2 = 2;
/// val2 = 3; // error: re-assignment of immutable variable
/// ```
///
/// This also lets you modify fields of a struct that you own
/// ```
/// struct Monster { health: u8 }
///
/// let mut orc = Monster { health: 93 };
/// orc.health -= 54;
///
/// let goblin = Monster { health: 28 };
/// goblin.health += 10; // error: cannot assign to immutable field
/// ```
///
///
/// `foo: &mut T` means you have a variable that refers to `(&)` a value and you are
/// allowed to change `(mut)` the **referred value** (including fields, if it is a struct):
///
/// ```
/// let val1 = &mut 2;
/// *val1 = 3; // OK
///
/// let val2 = &2;
/// *val2 = 3; // error: cannot assign to immutable borrowed content
/// ```
/// Note that `&mut` only makes sense with a reference - `foo: mut T` is not valid syntax.
/// You can also combine the two qualifiers `(let mut a: &mut T)`, when it makes sense.
///
///
/// FYI:
/// There are four possible combinations of `mut` in references and patterns:
///
/// ```
///     a: &T      // immutable binding of immutable reference
/// mut a: &T      // mutable binding of immutable reference
///     a: &mut T  // immutable binding of mutable reference
/// mut a: &mut T  // mutable binding of mutable reference
/// ```

fn push98(v: &mut Vec<i32>) {
    v.push(98);
}

fn has_odd(v: &Vec<i32>) -> bool {
    v.iter().any(|e| e % 2 != 0)
}

fn main() {
    let mut a = vec![2,4,6,8];
    let b = &mut a;
    b.push(99);

    dbg!(&a);
}