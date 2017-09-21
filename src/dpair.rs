use typed::*;
use arrow::*;
use darrow::*;

/// Type of dependent pairs
pub struct Sigma<T1:Typed,Tf:Typed<T=Arrow<T1,Star>>>(T1,Tf);
impl<T1:Typed,Tf:Typed<T=Arrow<T1,Star>>> Typed for Sigma<T1,Tf> {type T=Star;}

/// Dependent pairs of values
pub struct DPair<Tf,A,B>(Tf,A,B) where
	Tf:Typed<T=Arrow<A::T,Star>>,
	Tf:AbsArrow<T1=A::T,T2=Star>+Func<A>,
	A:Typed, A::T:Typed,
	B:Typed<T=Tf::F>,
;
impl<T1,Tf,A,B> Typed for DPair<Tf,A,B> where
	T1:Typed,
	Tf:Typed<T=Arrow<A::T,Star>>,
	Tf:AbsArrow<T1=A::T,T2=Star>+Func<A>,
	A:Typed<T=T1>,
	B:Typed<T=Tf::F>,
{type T=Sigma<T1,Tf>;}

// pub struct Dfst;
// impl<A:Typed,Tf:Typed> Typed for Dfst where
// 	A::T:Typed,
// 	Tf:AbsArrow<T1=A::T,T2=Star>+Func<A>,
// {type T=Pi<Sigma<A::T,Tf>,Tf::F>;}
