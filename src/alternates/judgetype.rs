//! Goal
//! ----
//! Using Rust type system as theorem prover for type theory,
//! or at least as a verifier
//!
//! Use Rust types as object language, have a meta-language
//! Both types and values are part of the object language
//!
//! Suggestion
//! ----------
//! traits as types
//! ```
//! trait Bool {}
//! struct True;
//! impl Bool for True {}
//! ```
//!
//! Problem
//! -------
//! We want a clear distinction between theory and meta-theory. Using traits in
//! the theory will muddle it up.
//!
//! Suggestion
//! ----------
//! one trait as typing provider
//! ```
//! trait Typed {type T;}
//! struct Bool;
//! struct True;
//! impl Typed for True{type T = Bool;}
//! ```
//! 
//! Problem
//! -------
//! Might be too simplistic? We need judgement forms to match standard theory
//! 
//! Suggestion
//! ----------
//! Use trait as typing judgement, need some kind of context
//! 
//! ```
//! trait Context {}
//! impl Context for () {}
//! 
//! trait Type<V,T> {}
//! 
//! struct Bool;
//! struct True;
//! 
//! impl<C:Context> Type<True,Bool> for C {}
//! ```
//! 
//! 
//! 
//! 
//! 

// Context
trait Context {}
impl Context for () {}

// Judgement Forms
trait Type<V,T> {}

// Types
struct Unit;
struct Product<T1,T2>(T1,T2);
struct Sum<T1,T2>(T1,T2);
struct Arrow<T1,T2>(T1,T2);

// Constructors
//struct ();
struct Pair<V1,V2>(V1,V2);
struct Injl<V>(V);
struct Injr<V>(V);


// Typing Judgement
impl<C:Context>
	Type<(),Unit> for C
where
	// axiom
{}
impl<C:Context,V1,V2,T1,T2>
	Type<Pair<V1,V2>,Product<T1,T2>> for C
where
	C : Type<V1,T1>,
	C : Type<V2,T2>,
{}
impl<C:Context,V,T1,T2>
	Type<Injl<V>,Sum<T1,T2>> for C
where
	C: Type<V,T1>
{}
impl<C:Context,V,T1,T2>
	Type<Injr<V>,Sum<T1,T2>> for C
where
	C: Type<V,T2>
{}

// Helpers
trait HasType<T> {}
impl<V,T>
	HasType<T> for V
where
	(): Type<V,T>
{}

//tests
trait TestHasType<T> : HasType<T> {}
// succeeds
impl TestHasType<Sum<Product<Unit,Unit>,Unit>> for Injl<Pair<(),()>> {}
// fails
// impl TestHasType<Unit> for Pair<(),()> {}
























