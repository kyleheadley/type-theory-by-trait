/// Property that represents a typing of the object its implemented for
pub trait Type<T:?Sized> {}
/// (Partial) function of one variable
pub trait Func<T1:?Sized,T2:?Sized,A:Type<T1>> {type F:Type<T2>;}
/// (Partial) function of two variables
pub trait Func2<T1:?Sized,T2:?Sized,T3:?Sized,A:Type<T1>,B:Type<T2>> {type F:Type<T3>;}
/// Judgment on one variable (predicate)
pub trait Judge<T:?Sized,A:Type<A>> {}
/// Judgment on two variables
pub trait Judge2<T1:?Sized,T2:?Sized,A:Type<T1>,B:Type<T2>> {}
