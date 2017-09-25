use typed::*;
use arrow::*;
use bool::*;
use nat::*;

// Syntax

pub trait Syntax<S> {}
pub struct Variable;
pub struct Term;
pub struct Type;
pub struct Value;
pub struct Context;

// Variables

// TODO: add equality requirements
/// Variables wrapped in Var<> for readability
pub struct Var<V>(V);
// Using numbers as variables
impl Syntax<Variable> for Var<Zero> {}
impl<N> Syntax<Variable> for Var<Succ<N>> where
	N:Typed<T=Nat>,
{}

// Terms

impl<V> Syntax<Term> for Var<V> {}

/// Lambdas [λx:T.t]
pub struct L<V,Ty,Tm>(V,Ty,Tm);
impl<V,Ty,Tm> Syntax<Term> for L<V,Ty,Tm> where
	V: Syntax<Variable>,
	Ty: Syntax<Type>,
	Tm: Syntax<Term>,
{}

// Applications are tuples for convenience
impl<A,B> Syntax<Term> for (A,B) where
	A: Syntax<Term>,
	B: Syntax<Term>,
{}

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

impl<V,Ty,Tm> Syntax<Value> for L<V,Ty,Tm> where
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

// Relations

pub trait Evaluation {type Result;}
pub trait Relation {}
pub trait Proven : Relation {}

// Typing

pub struct Typing<G,Tm>(G,Tm);

// T-Var
// TODO

// T-Abs
impl<G,X,T1,T2,Tm> Evaluation for Typing<G,L<X,T1,Tm>> where
	G: Syntax<Context>,
	X: Syntax<Variable>,
	T1: Syntax<Type>+Typed,
	T2: Syntax<Type>+Typed,
	Tm: Syntax<Term>,
	Typing<Bind<G,X,T1>,Tm> : Evaluation<Result=T2>,
{type Result=Arrow<T1,T2>;}

// T-App
impl<G,Tm1,Tm2,T1,T2> Evaluation for Typing<G,(Tm1,Tm2)> where
	G: Syntax<Context>,
	Tm1: Syntax<Term>,
	Tm2: Syntax<Term>,
	T1: Syntax<Type>+Typed,
	T2: Syntax<Type>+Typed,
	Typing<G,Tm1> : Evaluation<Result=Arrow<T1,T2>>,
	Typing<G,Tm2> : Evaluation<Result=T1>,
{type Result=T2;}

// T-True
impl<G> Evaluation for Typing<G,True> where
	G: Syntax<Context>,
{type Result=Bool;}

// T-False
impl<G> Evaluation for Typing<G,False> where
	G: Syntax<Context>,
{type Result=Bool;}
