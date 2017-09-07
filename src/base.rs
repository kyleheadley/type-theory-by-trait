/// Property that represents a typing of the object its implemented for
pub trait Type {type T:?Sized;}
/// Property for type of functions of one variable
pub trait Arrow {type T1; type T2;}
/// Property for type of functions of two variables
pub trait Arrow2 {type T1; type T2; type T3;}

/// (Partial) function of one variable
pub trait Func<A:Type<T=Self::T1>> : Arrow {type F:Type<T=Self::T2>;}
/// (Partial) function of two variables
pub trait Func2<A:Type<T=Self::T1>,B:Type<T=Self::T2>> : Arrow2 {type F:Type<T=Self::T3>;}


/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}

