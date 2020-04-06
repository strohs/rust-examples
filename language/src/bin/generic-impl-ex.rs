struct S; // Concrete type `S`

#[derive(Debug)]
struct GenericVal<T>(T,); // Generic type `GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl <T> GenericVal<T> {}

struct Val<T> {
    val: T
}

struct GenVal<T>{
    gen_val: T
}

// impl of Val
impl Val<f64> {
    fn value(&self) -> &f64 { &self.val }
}

// impl of GenVal for a generic type `T`
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn main() {
    let x = Val { val: 3.14 };
    let y = GenVal { gen_val: 8.6f32 };
    let gv = GenericVal("fooby");

    println!("{}, {}", x.value(), y.value());
    println!("{:?}", gv);
}