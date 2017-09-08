use base::*;
use arrow::*;

/// Type of dependent functions
pub struct Pi<T1:Type,Tf:Type<T=Arrow<T1,Star>>>(T1,Tf);
impl<T1:Type,Tf:Type<T=Arrow<T1,Star>>> Type for Pi<T1,Tf> {type T=Star;}
/// Property used for defining dependent functions
pub trait DFuncType {type T1:Type; type Tf:Type<T=Arrow<Self::T1,Star>>;}
impl<T1:Type,Tf:Type<T=Arrow<T1,Star>>,P:Type<T=Pi<T1,Tf>>> DFuncType for P {type T1=T1; type Tf=Tf;}

/// Dependent function
pub trait DFunc<A:Type<T=<Self::Tf as FuncType>::T1>> : DFuncType where
	Self::Tf: Func<A>,
{type D:Type<T=<Self::Tf as Func<A>>::F>;}
