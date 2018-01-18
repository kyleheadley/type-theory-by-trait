/// I don't know how useful this will be
/// It could be used with a macro to guarentee all varients are used
/// It doesn't work with constructors, it needs functions
/// More experimenting is needed
///
/// This probably won't work as is: constructors take arguments,
/// and this is not yet sophisticated enough to deal with them.
/// Maybe when Rust gets parameters for associated types this will
/// be easier.

use typed::*;
use arrow::*;
use bool::*;
use nat::*;
use util::*;

struct Gen;

trait EnumBool {
	type False;
	type True;
}

impl EnumBool for Gen {
	type False = False;
	type True = True;
}

// struct Not;
// impl Typed for Not {type T=Arrow<Bool,Bool>; fn reflect() -> String {format!("not()")}}
// impl EnumBool for Not {
// 	type False = <Gen as EnumBool>::True;
// 	type True = <Gen as EnumBool>::False;
// }
// impl Func<<Gen as EnumBool>::False> for Not {
// 	type F = <Not as EnumBool>::False;
// }
// impl Func<<Gen as EnumBool>::True> for Not {
// 	type F = <Not as EnumBool>::True;
// }

struct Not;
impl Typed for Not {type T=Arrow<Bool,Bool>; fn reflect() -> String {format!("not()")}}
impl Func<False> for Not {type F=True;}
impl Func<True> for Not {type F=False;}
impl EnumBool for Not {
	type False = <Not as Func<False>>::F;
	type True = <Not as Func<True>>::F;
}

type F = <Gen as EnumBool>::False;
type T = <Gen as EnumBool>::True;
type NF = <Not as EnumBool>::False;
type NT = <Not as Func<T>>::F;


trait EnumNat {
	type Zero;
	type Succ : Typed<T=Arrow<Nat,Nat>>;
}

impl EnumNat for Gen {
	type Zero = Zero;
	type Succ = SuccFn;
}

struct Plus1;
impl Typed for Plus1 {type T=Arrow<Nat,Nat>; fn reflect()->String{format!("+1")}}
impl Func<Zero> for Plus1 {type F=Succ<Zero>;}
//impl<N:Typed<T=Nat>> Func<Succ<N>> for Plus1 {type F=Succ<Succ<N>>;}
impl EnumNat for Plus1 {
	type Zero = <Plus1 as Func<Zero>>::F;
	type Succ = Comp<Plus1,SuccFn>;
}
