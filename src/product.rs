use base::*;

pub struct Product<T1,T2>(pub T1, pub T2);

pub struct Pair<A,B>(pub A, pub B);
impl<T1,T2,A:Type<T1>,B:Type<T2>> Type<Product<T1,T2>> for Pair<A,B> {}

pub struct UnitType {}

pub struct Unit;
impl Type<UnitType> for Unit {}

//pub trait ProductRecType<>;
