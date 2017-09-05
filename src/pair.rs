use base::*;

pub trait ProductType<A,B> {}

pub struct Pair<A,B>(pub A, pub B);
impl<A,B> ProductType<A,B> for Pair<A,B> {}

pub struct Unit;
impl Type for Unit {}

//pub trait ProductRecType<>;
