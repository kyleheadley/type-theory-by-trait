use typed::*;
use arrow::*;
use bool::*;
use nat::*;

trait View {
	fn view() -> String {format!("unknown")}
}
impl<T:Typed> View for T {fn view() -> String {T::reflect()}}

// Syntax

pub trait Syntax<S> {}
pub struct Variable;
pub struct Term;
pub struct Type;
pub struct Value;
pub struct Context;

// Variables

/// Variables wrapped in Var<> for readability
pub struct Var<V>(V);
// Using numbers as variables
impl Syntax<Variable> for Var<Zero> {}
impl<N> Syntax<Variable> for Var<Succ<N>> where
	N:Typed<T=Nat>,
{}
impl<V:View> View for Var<V> {fn view() -> String {format!("var({})",V::view())}}
/// Equality for variables (meta func)
pub struct VarEqual;
impl<N1,N2> MetaFunc2<Var<N1>,Var<N2>> for VarEqual where
	Var<N1>: Syntax<Variable>,
	Var<N2>: Syntax<Variable>,
	N1:Typed<T=Nat>,
	N2:Typed<T=Nat>,
	IsEqual: Func2<N1,N2>
{type M=<IsEqual as Func2<N1,N2>>::F;}

// Terms

impl<V> Syntax<Term> for Var<V> {}

/// Lambdas [λx:T.t]
pub struct Lam<V,Ty,Tm>(V,Ty,Tm);
impl<V,Ty,Tm> Syntax<Term> for Lam<V,Ty,Tm> where
	V: Syntax<Variable>,
	Ty: Syntax<Type>,
	Tm: Syntax<Term>,
{}
impl<V:View,Ty:View,Tm:View> View for Lam<V,Ty,Tm> {
	fn view() -> String {format!("λ{}:{}.{}",V::view(),Ty::view(),Tm::view())}
}

// Applications are tuples for convenience
impl<A,B> Syntax<Term> for (A,B) where
	A: Syntax<Term>,
	B: Syntax<Term>,
{}
impl<A:View,B:View> View for (A,B) {fn view() -> String {format!("({} {})",A::view(),B::view())}}

// base values are terms
impl Syntax<Term> for True {}
impl Syntax<Term> for False {}

// Types

// import Bool
impl Syntax<Type> for Bool {}

// import Arrow
impl<A,B> Syntax<Type> for Arrow<A,B> where
	A:Syntax<Type>+Typed,
	B:Syntax<Type>+Typed,
{}

// Values

impl<V,Ty,Tm> Syntax<Value> for Lam<V,Ty,Tm> where
	V: Syntax<Variable>,
	Ty: Syntax<Type>,
	Tm: Syntax<Term>,
{}

// boolean values
impl Syntax<Value> for True {}
impl Syntax<Value> for False {}

// contexts

pub struct EmptyCtxt;
impl Syntax<Context> for EmptyCtxt {}

pub struct Bind<G,V,T>(G,V,T);
impl<G,V,T> Syntax<Context> for Bind<G,V,T> where
	G: Syntax<Context>,
	V: Syntax<Variable>,
	T: Syntax<Type>,
{}

// Γ(x)
pub trait Lookup<X> {type L;}
impl<X,G,V,T> Lookup<X> for Bind<G,V,T> where
	X: Syntax<Variable>,
	G: Syntax<Context>,
	V: Syntax<Variable>,
	T: Syntax<Type>,
	VarEqual: MetaFunc2<X,V>,
	Bind<G,V,T>: LookupFinish<<VarEqual as MetaFunc2<X,V>>::M,X>,
{type L=<Bind<G,V,T> as LookupFinish<<VarEqual as MetaFunc2<X,V>>::M,X>>::L;}
pub trait LookupFinish<B,X> {type L;}
impl<X,G,V,T> LookupFinish<True,X> for Bind<G,V,T> where
	X: Syntax<Variable>,
	G: Syntax<Context>,
	V: Syntax<Variable>,
	T: Syntax<Type>,
{type L=T;}
impl<X,G,V,T> LookupFinish<False,X> for Bind<G,V,T> where
	X: Syntax<Variable>,
	G: Syntax<Context>,
	V: Syntax<Variable>,
	T: Syntax<Type>,
	G: Lookup<X>,
{type L=<G as Lookup<X>>::L;}

// Judgements

pub trait Evaluation {type Result;}
pub trait Relation {}
pub trait Proven : Relation {}

// Typing

pub struct Typing<G,Tm>(G,Tm);

// // T-Var
// impl<G,V> Evaluation for Typing<G,V> where
// 	G: Syntax<Context>,
// 	V: Syntax<Variable>,
// 	G: Lookup<V>
// {type Result=<G as Lookup<V>>::L;}

// T-Abs
impl<G,X,T1,T2,Tm> Evaluation for Typing<G,Lam<X,T1,Tm>> where
	G: Syntax<Context>,
	X: Syntax<Variable>,
	T1: Syntax<Type>+Typed,
	T2: Syntax<Type>+Typed,
	Tm: Syntax<Term>,
	Typing<Bind<G,X,T1>,Tm> : Evaluation<Result=T2>,
{type Result=Arrow<T1,T2>;}

// // T-App
// impl<G,Tm1,Tm2,T1,T2> Evaluation for Typing<G,(Tm1,Tm2)> where
// 	G: Syntax<Context>,
// 	Tm1: Syntax<Term>,
// 	Tm2: Syntax<Term>,
// 	T1: Syntax<Type>+Typed,
// 	T2: Syntax<Type>+Typed,
// 	Typing<G,Tm1> : Evaluation<Result=Arrow<T1,T2>>,
// 	Typing<G,Tm2> : Evaluation<Result=T1>,
// {type Result=T2;}

// // T-True
// impl<G> Evaluation for Typing<G,True> where
// 	G: Syntax<Context>,
// {type Result=Bool;}

// // T-False
// impl<G> Evaluation for Typing<G,False> where
// 	G: Syntax<Context>,
// {type Result=Bool;}



// Tests

// type R1 = <Typing<EmptyCtxt,True> as Evaluation>::Result;

// #[test] fn show() {println!("{}",R1::view());}

// fn whattype<Tm:View>(tm:Tm) where
// 	Tm: Syntax<Term>,
// 	Typing<EmptyCtxt,Tm>: Evaluation,
// 	<Typing<EmptyCtxt,Tm> as Evaluation>::Result: Typed,
// {println!("{}:{}", Tm::view(), <<Typing<EmptyCtxt,Tm> as Evaluation>::Result>::reflect());}

// trait Same<X> {}
// impl<X> Same<X> for X {}

// fn judgetype<Tm,Ty>(tm:Tm,ty:Ty) where
// 	Tm: Syntax<Term>,
// 	Ty: Syntax<Type>,
// 	Typing<EmptyCtxt,Tm>: Evaluation,
// 	<Typing<EmptyCtxt,Tm> as Evaluation>::Result: Same<Ty>
// {}

// fn judgetypectxt<G,Tm,Ty>(g:G,tm:Tm,ty:Ty) where
// 	G: Syntax<Context>,
// 	Tm: Syntax<Term>,
// 	Ty: Syntax<Type>,
// 	Typing<G,Tm>: Evaluation,
// 	<Typing<G,Tm> as Evaluation>::Result: Same<Ty>
// {}

// #[test] fn checktypes() {
// 	// whattype(True);
// 	// judgetypectxt(
// 	// 	Bind(Bind(EmptyCtxt,Var(Zero),Bool),Var(Succ(Zero)),Arrow(Bool,Bool)),
// 	// 	Var(Zero),
// 	// 	Bool
// 	// );
// }

