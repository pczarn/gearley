// struct PartialAny<'a, T, const V: usize, const B: usize, const M: usize> {
//     value: [T; V],
//     borrow: [&'a T; B],
//     borrow_mut: [&'a mut T; M],
// }

// struct Val;
// struct Ref;
// struct Mut;

// trait Kind {
//     const V: usize;
//     const B: usize;
//     const M: usize;
// }

// impl Kind for Val {
//     const V: usize = 1;
//     const B: usize = 0;
//     const M: usize = 0;
// }

// type Partial<T> = PartialAny<'static, T, 1, 0, 0>;
// type PartialRef<'a, T> = PartialAny<'a, T, 0, 1, 0>;
// type PartialMut<'a, T> = PartialAny<'a, T, 0, 0, 1>;

// macro_rules! partial {
//     ($name:ident) => {
//         const concat_ident!($name, 0): usize, const concat_ident!($name, 1): usize, const concat_ident!($name, 2): usize
//     };
// }

// struct Graph<const A: usize> {
//     neighbors: partial!(A, Vec<Vec<usize>>),
//     colors: partial!(B, Vec<usize>),
//     weights: partial!(C, Vec<f32>),
// }

// struct PartialGraph {

// }

// impl<T> From<T> for Partial<T> {
//     fn from(value: T) -> Self {
//         PartialAny {
//             value: [value],
//             borrow: [],
//             borrow_mut: [],
//         }
//     }
// }

// impl Graph {
//     fn add_color_to_weight(self: &Graph) {

//     }

// }

// impl Graph {
//     fn new() -> Self {
//         Graph {
//             neighbors: vec![].into(),
//             colors: vec![].into(),
//             weights: vec![].into(),
//         }
//     }
// }

// fn test() {
//     let mut g = Graph::new();
// }
