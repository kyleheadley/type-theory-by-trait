/// Property that represents a typing of the object its implemented for
pub trait Type {type T:?Sized;}
/// Temporary type-of-types
pub struct Star(());

/// Property for type of functions of one variable
pub trait Arrow {type T1; type T2;}
/// Property for type of functions of two variables
pub trait Arrow2 {type T1; type T2; type T3;}

/// (Partial) function of one variable
pub trait Func<A:Type<T=Self::T1>> : Arrow {type F:Type<T=Self::T2>;}
/// (Partial) function of two variables
pub trait Func2<A:Type<T=Self::T1>,B:Type<T=Self::T2>> : Arrow2 {type F:Type<T=Self::T3>;}

/// Type of Dependent functions
pub trait PiType {
	type T1;
	type F:Arrow<T1=Self::T1, T2=Star>;
}

/// Dependent function
pub trait DFunc<A:Type<T=Self::T1>> : PiType where
	Self::F: Func<A>,
{type D:Type<T=<Self::F as Func<A>>::F>;}

/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}

/// Composite Function [ A(B(x)) ]
pub struct Comp<A:Arrow,B:Arrow<T2=A::T1>>(A,B);
impl<A:Arrow,B:Arrow<T2=A::T1>> Arrow for Comp<A,B> {type T1=B::T1; type T2=A::T2;}
impl<A:Arrow,B:Arrow<T2=A::T1>,X:Type<T=B::T1>> Func<X> for Comp<A,B> where
	B: Func<X>,
	A: Func<<B as Func<X>>::F>,
{type F=<A as Func<<B as Func<X>>::F>>::F;}

