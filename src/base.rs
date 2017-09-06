/// Property that represents a typing of the object its implemented for
pub trait Type<T> {}
/// (Partial) untyped function of one variable
pub trait Func<A> {type F;}
/// (Partial) untyped function of two variables
pub trait Func2<A,B> {type F;}
/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}

/// Type of functions
pub trait Arrow<T1,T2> {}