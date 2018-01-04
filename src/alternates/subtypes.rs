//! Demo of static subtyping
//!
//! Use trait objects to get dynamic subtyping,
//! but casting to a subtype is not explored here

////////////////////////////////////////////////////////
// Define Base type, interface, and what is overloadable
////////////////////////////////////////////////////////

/// The main type that will be extended
struct Basetype<E:BaseExtend> {
    data: usize,
    ext: E::Ext,
}
/// Additional data and overloadable functions for subtypes
trait BaseExtend : Sized { 
	/// data to add to the base data
	type Ext;
	/// the "overloadable" function from the base interface
	fn sub_info(s:&Basetype<Self>) -> String { String::from("None") }
}
/// Convenience for code clarity
///
/// allows:     fn foo<B:BaseInterface>(b:B) {}
/// instead of: fn foo<E:BaseExtend>(b:Basetype<E>) {}
trait BaseInterface {
	fn get_data(&self) -> usize;
	fn sub_info(&self) -> String;
}
impl<E:BaseExtend> BaseInterface for Basetype<E>{
	/// not overloadable
    fn get_data(&self) -> usize { self.data }
    /// call the overloadable version
    fn sub_info(&self) -> String { E::sub_info(self) }
}

impl BaseExtend for () {
	type Ext = ();
	// not including functions uses the default defined above
}
/// non-extended version of the base type
type Base = Basetype<()>;
impl Base {
	/// function unique to the base type
	fn new(a:usize) -> Base {Basetype{ data:a, ext:()}}
}

/////////////////////////////////////////////////////////////////
// Define some subtypes and their overloaded and unique functions
/////////////////////////////////////////////////////////////////

/// marker for a subtype
struct Subtype1 {}
impl BaseExtend for Subtype1 {
	type Ext = String;
	fn sub_info(s:&Basetype<Self>) -> String { s.ext.clone() }
}
/// a subtype of the base type
type Sub1 = Basetype<Subtype1>;
impl Sub1 {
	/// function unique to this sub type
	fn new(a:usize,b:String) -> Sub1 {Basetype{ data: a, ext: b }}
}

/// marker for a subtype
struct Subtype2 {}
impl BaseExtend for Subtype2 {
	type Ext = usize;
	fn sub_info(s:&Basetype<Self>) -> String { s.ext.to_string() }
}
/// a subtype of the base type
type Sub2 = Basetype<Subtype2>;
impl Sub2 {
	/// function unique to this sub type
	fn new(a:usize,b:usize) -> Sub2 {Basetype{ data: a, ext: b }}
}

///////////////////////////////////
// Define how a multi-subtype works
///////////////////////////////////

/// marker for a way to make a multi-sub type
struct Union {}
impl<A:BaseExtend,B:BaseExtend> BaseExtend for (A,B) where
	A::Ext: ToString, B::Ext: ToString,
{
	type Ext = (A::Ext,B::Ext);
	fn sub_info(s:&Basetype<Self>) -> String {
		format!("{} - {}",s.ext.0.to_string(),s.ext.1.to_string())
	}
}
impl Union {
	/// function unique to this union type
	fn new<A:BaseExtend,B:BaseExtend>(a:Basetype<A>,b:Basetype<B>) -> Basetype<(A,B)> where
		A::Ext: ToString, B::Ext: ToString,
	{
		Basetype{ data: a.data + b.data, ext: (a.ext,b.ext)}
	}
}

////////////////////////
// Define a Sub-Sub type
////////////////////////

struct Subtype<E:SubExtend>{
	data: usize,
	ext: E::Ext,
}
trait SubExtend : Sized {
	type Ext;
	fn subsub_info(s:&Basetype<Subtype<Self>>) -> String { String::from("None") }
	fn say_more(s:&Basetype<Subtype<Self>>) -> String { String::from("more!") } 
}
impl<E:SubExtend> BaseExtend for Subtype<E> {
	type Ext = Subtype<E>;
	fn sub_info(s:&Basetype<Self>) -> String { format!("Sub{}: {}", s.ext.data, E::subsub_info(s)) }
}
impl<E:SubExtend> Basetype<Subtype<E>> {
	fn say_more(&self) -> String { E::say_more(self) } 
}
impl SubExtend for () {
	type Ext = ();
}
impl Basetype<Subtype<()>>{
	fn new() -> Self { Basetype{data:0,ext:Subtype{data:1,ext:()}} }
}
struct Defiant {}
impl SubExtend for Defiant {
	type Ext = String;
	fn subsub_info(s:&Basetype<Subtype<Self>>) -> String { s.ext.ext.clone() }
	fn say_more(s:&Basetype<Subtype<Self>>) -> String { String::from("I don't want to!") } 
}
impl Basetype<Subtype<Defiant>> {
	fn new() -> Self { Basetype{data:51256,ext:Subtype{data:9289,ext:String::from("blah, blah")}} }
}

/////////////////////////
// Show that it all works
/////////////////////////

/// function that takes any base or sub type
///
/// alternatly, it could be defined more specifically:
/// fn get_sub<E:BaseExtend>(b:Basetype<E>) -> String {b.sub_info()}
/// which would allow a where clause to restrict E further
fn get_sub<B:BaseInterface>(b:&B) -> String {b.sub_info()}

#[test]
fn main() {
    let a = Base::new(2);
    println!("a base data: {}",a.get_data());
    println!("a ext data: {}",get_sub(&a));
    let b = Sub1::new(2,String::from("yes"));
    println!("b base data: {}",b.get_data());
    println!("b ext data: {}",get_sub(&b));
    let c = Sub2::new(2,3);
    println!("c base data: {}",c.get_data());
    println!("c ext data: {}",get_sub(&c));
    let d = Union::new(Sub2::new(2,4),Sub2::new(6,8));
    println!("d base data: {}",d.get_data());
    println!("d ext data: {}",get_sub(&d));
    let e = Basetype::<Subtype<Defiant>>::new();
    println!("e base data: {}",e.get_data());
    println!("e ext data: {}",get_sub(&e));
    println!("e, say more: {}", e.say_more());
}
