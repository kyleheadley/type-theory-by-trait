use base::*;
use arrow::*;

/// Type of Dependent functions
pub trait PiType {
	type T1;
	type F:FuncType<T1=Self::T1, T2=Star>;
}

/// Dependent function
pub trait DFunc<A:Type<T=Self::T1>> : PiType where
	Self::F: Func<A>,
{type D:Type<T=<Self::F as Func<A>>::F>;}

