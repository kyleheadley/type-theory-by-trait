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

impl<N> Syntax<Term> for Var<N> {}

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
impl Context<Value> for True {}
impl Context<Value> for False {}

// contexts

struct EmptyCtx;
impl Syntax<Context> for EmptyCtx {}

struct Bind<G,V,T>(G,V,T);
impl<G,V,T> Syntax<Context> for Bind<G,V,T> where
	G: Syntax<Context>,
	V: Syntax<Variable>,
	T: Syntax<Type>,
{}

