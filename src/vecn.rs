pub struct VecN<T, const I: usize> {
    pub a: [T; I],
}

enum VecNKinds<T> {
    Colour(VecN<T, 3>),
    Point(VecN<T, 3>),
}
