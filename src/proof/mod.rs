// Meta-language natural number
trait Counter {}
struct Base;
struct Next<C:Counter>(C);
impl Counter for Base {}
impl<C:Counter> Counter for Next<C> {}
// Meta-language functions over counters
trait CounterFn2<C1:Counter,C2:Counter> { type C:Counter; }
struct Max;
impl<I2:Counter> CounterFn2<Base,I2> for Max { type C = I2; }
impl<I1:Counter> CounterFn2<Next<I1>,Base> for Max { type C = Next<I1>; }
impl<I1:Counter,I2:Counter> CounterFn2<Next<I1>,Next<I2>> for Max where
	Max : CounterFn2<I1,I2>
{ type C = Next<<Max as CounterFn2<I1,I2>>::C>;}

// Type and universe level of an object
trait Typed {
	type U : Counter;
	type T : Typed<U=Next<Self::U>>;
}
// Simplified type bound, avoid early well-formedness check
// (Accessor trait)
trait Typ { type T; }
impl<A:Typed> Typ for A { type T = A::T; } 

// Universe of types
struct Type<U:Counter>(U);
impl<U:Counter> Typed for Type<U> {
	type U = U;
	type T = Type<Next<U>>;
}

// Abstract bound for types
trait AbstractType {type U: Counter;}
impl<U:Counter> AbstractType for Type<U> {type U = U;}

// Universe of small types
type UTypes = Next<Base>;
// Type of small types
type TTypes = Type<Next<Next<Base>>>;

// Simple(non-dependent) function type
struct Arrow<I,O>(I,O);
impl<IU,OU,I,O> Typed for Arrow<I,O> where
	IU : Counter,
	OU : Counter,
	I : Typed<U=IU>,
	O : Typed<U=OU>,
	Max : CounterFn2<IU,OU>,
{
	type U = <Max as CounterFn2<IU,OU>>::C;
	type T = Type<Next<<Max as CounterFn2<IU,OU>>::C>>;
}

// Abstract bound for arrows
trait AbstractArrow {type In; type Out;}
impl<I:Typed,O:Typed> AbstractArrow for Arrow<I,O> {
	type In=I; type Out=O;
}

// Pi/Dependent function types Π(a:α)β a,
//	where a is defined later and β:α->Type<u>
struct DArrow<A,B>(A,B) where
	A : Typed, B : Typed,
	B::T : AbstractArrow<In=A>,
	<B::T as AbstractArrow>::Out : AbstractType
;
// TODO:
// impl Typed for DArrow<A,B> where
// 	A : Typed, B : Typed,
// 	B::T : AbstractArrow<In=A>,
// 	<B::T as AbstractArrow>::Out : AbstractType
// {
// 	type U = ;
// 	type T = ;
// }

// Abstract bound for dependent arrows
trait AbstractDArrow {type In; type Out; }
impl<I:Typed,O:Typed> AbstractDArrow for DArrow<I,O> where
	O::T : AbstractArrow<In=I>,
	<O::T as AbstractArrow>::Out : AbstractType,
{ type In=I; type Out=O; }

// Equality Type
struct Eq<A,B>(A,B);
impl<A,B> Typed for Eq<A,B> where
	A : Typed,
	B : Typed<U=A::U,T=A::T>,
{
	type U = Next<A::U>;
	type T = Type<Next<Next<A::U>>>;
}

// The element of equality type
struct Refl<A>(A);
impl<A:Typed> Typed for Refl<A> { type U = A::U; type T = Eq<A,A>; }

// TODO: symm, trans


// Type of natural numbers
struct Nat;
impl Typed for Nat { type U=UTypes; type T=TTypes; }

// The natural numbers
struct Z;
struct S<N:Typ<T=Nat>>(N);
impl Typed for Z { type U=Base; type T = Nat;}
impl<N> Typed for S<N> where
	N : Typ<T=Nat>
{
	type U=Base;
	type T=Nat;
}












