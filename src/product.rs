use base::*;

pub struct Product<T1,T2>(T1, T2);

pub struct Pair<A,B>(pub A, pub B);
impl<T1,T2,A:Type<T=T1>,B:Type<T=T2>>
Type for Pair<A,B> {type T=Product<T1,T2>;}

pub struct UnitType(());

pub struct Unit;
impl Type for Unit {type T=UnitType;}

//pub trait ProductRecType<>;
