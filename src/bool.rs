use typed::*;
use arrow::*;

/// Boolean type
pub struct Bool;
impl Typed for Bool {
	fn reflect() -> String {format!("Bool")}
	type T=Star;
}

/// True : Bool
pub struct True;
impl Typed for True {
	fn reflect() -> String {format!("true")}
	type T=Bool;
}
/// False : Bool
pub struct False;
impl Typed for False {
	fn reflect() -> String {format!("false")}
	type T=Bool;
}

/// Disjunction function for booleans
pub struct Or;
impl Typed for Or {
	fn reflect() -> String {format!("or()")}
	type T=Arrow2<Bool,Bool,Bool>;
}
impl Func2<True,True> for Or {type F=True;}
impl Func2<True,False> for Or {type F=True;}
impl Func2<False,True> for Or {type F=True;}
impl Func2<False,False> for Or {type F=False;}
