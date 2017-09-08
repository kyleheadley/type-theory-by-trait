use base::*;

/// Type of pairs
pub struct Product<T1:Type,T2:Type>(T1,T2);
impl<A:Type,B:Type> Type for Product<A,B> {type T=Star;}

/// Pairs of values
pub struct Pair<A:Type,B:Type>(A,B);
impl<T1:Type,T2:Type,A:Type<T=T1>,B:Type<T=T2>> Type for Pair<A,B> {type T=Product<T1,T2>;}

/// Type of empty value
pub struct UnitType;
impl Type for UnitType {type T=Star;}

/// The empty value
pub struct Unit;
impl Type for Unit {type T=UnitType;}

//pub trait ProductRecType<>;
