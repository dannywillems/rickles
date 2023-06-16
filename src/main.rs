use std::marker::PhantomData;

// Theory about naturals
trait Nat {}
impl Nat for () {}
struct Succ<N: Nat> { _s: PhantomData<N>}
impl<N: Nat> Nat for Succ<N> {}

struct Empty;
trait List {}

impl List for Empty { }

struct Vector<T, N: Nat> {
    v: Vec<T>,
    _n: PhantomData<N>
}

struct Head<N: Nat, V, T: List> {
    tail: T,
    head: V,
    _n: PhantomData<N>
}

impl Empty {
    fn push<V>(v: V) -> Head<Succ<()>, V, Empty> {
        Head {
            tail: Empty,
            head: v,
            _n: PhantomData
        }
    }
}

// impl <N: Nat, V, T:List> for Head<N: Nat, V, T> {
//     fn head() -> Head<Succ<()>, V, Empty> {}
//     fn push()
// }


fn main() {
    println!("Hello, world!");
}
