use base::*;

pub trait Product<T1,T2> {}

pub struct Pair<A,B>(pub A, pub B);
impl<T1,T2,A:Type<T1>,B:Type<T2>> Product<T1,T2> for Pair<A,B> {}

pub trait UnitType {}
impl<U:Type<UnitType>> UnitType for U {}

pub struct Unit;
impl Type<UnitType> for Unit {}

//pub trait ProductRecType<>;
