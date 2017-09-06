/// Property that represents a typing of the object its implemented for
pub trait Type<T> {}

/// Type of functions of one variable
pub struct Arrow<T1,T2>(T1, T2);
/// Type of functions of two variables
pub struct Arrow2<T1,T2,T3>(T1, T2, T3);

/// (Partial) function of one variable
pub trait Func<T1,T2,A:Type<T1>> : Type<Arrow<T1,T2>> {type F:Type<T2>;}
/// (Partial) function of two variables
pub trait Func2<T1,T2,T3,A:Type<T1>,B:Type<T2>> : Type<Arrow2<T1,T2,T3>> {type F:Type<T3>;}

/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}

