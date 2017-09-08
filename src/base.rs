/// Property that represents a typing of the object its implemented for
pub trait Type {type T;}
/// Temporary type-of-types
pub struct Star;

/// Type of functions of one variable
pub struct Arrow<T1,T2>(T1,T2);
impl<T1,T2> Type for Arrow<T1,T2> {type T=Star;}
pub trait FuncType {type T1; type T2;}
impl<T1,T2,A:Type<T=Arrow<T1,T2>>> FuncType for A {type T1=T1; type T2=T2;}
/// Type of functions of two variables
pub struct Arrow2<T1,T2,T3>(T1,T2,T3);
impl<T1,T2,T3> Type for Arrow2<T1,T2,T3> {type T=Star;}
pub trait Func2Type {type T1; type T2; type T3;}
impl<T1,T2,T3,A:Type<T=Arrow2<T1,T2,T3>>> Func2Type  for A {type T1=T1; type T2=T2; type T3=T3;}

/// (Partial) function of one variable
pub trait Func<A:Type<T=Self::T1>> : FuncType {type F:Type<T=Self::T2>;}
/// (Partial) function of two variables
pub trait Func2<A:Type<T=Self::T1>,B:Type<T=Self::T2>> : Func2Type {type F:Type<T=Self::T3>;}

// untyped Func works as an alternate, but with no constraints
// pub trait Func<A> {type F;}
// pub trait Func2<A,B> {type F;}

/// Type of Dependent functions
pub trait PiType {
	type T1;
	type F:FuncType<T1=Self::T1, T2=Star>;
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
pub struct Comp<A:FuncType,B:FuncType<T2=A::T1>>(A,B);
impl<A:FuncType,B:FuncType<T2=A::T1>> Type for Comp<A,B> {type T=Arrow<B::T1,A::T2>;}
impl<A:FuncType,B:FuncType<T2=A::T1>,X:Type<T=B::T1>> Func<X> for Comp<A,B> where
	B: Func<X>,
	A: Func<<B as Func<X>>::F>,
{type F=<A as Func<<B as Func<X>>::F>>::F;}

/// Constant function from any type T to V
pub struct Const<T,V>(T,V);
impl<T1,T2,V:Type<T=T2>> Type for Const<T1,V> {type T=Arrow<T1,T2>;}
impl<T1,A:Type<T=T1>,T2,V:Type<T=T2>> Func<A> for Const<T1,V> {type F=V;}
