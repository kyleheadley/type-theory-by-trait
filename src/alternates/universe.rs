// Meta-language natural number
trait Counter {}
struct Base;
struct Next<C:Counter>(C);
impl Counter for Base {}
impl<C:Counter> Counter for Next<C> {}
// trait HigherThan<C:Counter> {}
// impl<C:Counter> HigherThan<Base> for Next<C> {}
// impl<C1:Counter,C2:Counter>
// 	HigherThan<Next<C1>> for Next<C2>
// where
// 	C2: HigherThan<C1>
// {}

// Typing level
trait InUniverse {type Level : Counter;}

macro_rules! def_value {($x:ident) => (
	struct $x;
	impl InUniverse for $x {type Level = Base;}
)}
macro_rules! def_type {($x:ident) => (
	struct $x;
	impl InUniverse for $x {type Level = Next<Base>;}
)}
macro_rules! def_kind {($x:ident) => (
	struct $x;
	impl InUniverse for $x {type Level = Next<Next<Base>>;}
)}

// Inherent Types
trait Typed : InUniverse {type T : InUniverse<Level=Next<Self::Level>>;}

// // Star
// def_kind![Star];
// macro_rules! def_star {($x:ident) => (
// 	def_type![$x];
// 	impl Typed for $x {type T = Star;}
// )}

// Bool
def_type![Bool];
macro_rules! def_bool {($x:ident) => (
	def_value![$x];
	impl Typed for $x {type T = Bool;}
)}
def_bool![True];
def_bool![False];
