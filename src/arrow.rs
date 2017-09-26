use typed::*;

/// Type of functions of one variable
pub struct Arrow<T1:Typed,T2:Typed>(pub T1,pub T2);
impl<T1:Typed,T2:Typed> Typed for Arrow<T1,T2> {
	fn reflect() -> String {format!("{}->{}",T1::reflect(),T2::reflect())}
	type T=Star;
}
/// Property used for defining functions
pub trait AbsArrow{type T1:Typed; type T2:Typed;}
impl<T1:Typed,T2:Typed,A:Typed<T=super::Arrow<T1,T2>>> AbsArrow for A {type T1=T1; type T2=T2;}
/// Type of functions of two variables
pub struct Arrow2<T1:Typed,T2:Typed,T3:Typed>(T1,T2,T3);
impl<T1:Typed,T2:Typed,T3:Typed> Typed for Arrow2<T1,T2,T3> {
	fn reflect() -> String {format!("{}->{}->{}",T1::reflect(),T2::reflect(),T3::reflect())}
	type T=Star;
}
/// Property used for defining functions
pub trait AbsArrow2 {type T1:Typed; type T2:Typed; type T3:Typed;}
impl<T1:Typed,T2:Typed,T3:Typed,A:Typed<T=Arrow2<T1,T2,T3>>> AbsArrow2 for A {type T1=T1; type T2=T2; type T3=T3;}

/// (Partial) function of one variable
pub trait Func<A:Typed<T=Self::T1>> : AbsArrow {type F:Typed<T=Self::T2>;}
/// (Partial) function of two variables
pub trait Func2<A:Typed<T=Self::T1>,B:Typed<T=Self::T2>> : AbsArrow2 {type F:Typed<T=Self::T3>;}

// untyped Func works as an alternate, but with no constraints
// pub trait Func<A> {type F;}
// pub trait Func2<A,B> {type F;}
