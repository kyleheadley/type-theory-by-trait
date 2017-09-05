use base::*;

pub trait ProductType<T1,T2> {}
impl<T1,T2,P:Type<ProductType<T1,T2>>> ProductType<T1,T2> for P {}

pub struct Pair<A,B>(pub A, pub B);
impl<T1,T2,A:Type<T1>,B:Type<T2>> ProductType<T1,T2> for Pair<A,B> {}

pub trait UnitType {}
impl<U:Type<UnitType>> UnitType for U {}

pub struct Unit;
impl Type<UnitType> for Unit {}

//pub trait ProductRecType<>;
