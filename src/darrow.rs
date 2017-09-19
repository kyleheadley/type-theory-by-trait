use typed::*;
use arrow::*;

/// Type of dependent functions
pub struct Pi<T1:Typed,Tf:Typed<T=Arrow<T1,Star>>>(T1,Tf);
impl<T1:Typed,Tf:Typed<T=Arrow<T1,Star>>> Typed for Pi<T1,Tf> {type T=Star;}
/// Property used for defining dependent functions
pub trait DFuncType {type T1:Typed; type Tf:Typed<T=Arrow<Self::T1,Star>>;}
impl<T1:Typed,Tf:Typed<T=Arrow<T1,Star>>,P:Typed<T=Pi<T1,Tf>>> DFuncType for P {type T1=T1; type Tf=Tf;}

/// Dependent function
pub trait DFunc<A:Typed<T=<Self::Tf as AbsArrow>::T1>> : DFuncType where
	Self::Tf: Func<A>,
{type D:Typed<T=<Self::Tf as Func<A>>::F>;}
