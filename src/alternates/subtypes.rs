trait Extend { type Extra; }
impl Extend for () { type Extra = (); }

struct Basetype<E:Extend>{
    data: usize,
    ext: E::Extra,
}
impl<E:Extend> Basetype<E>{
    fn get_data(&self) -> usize { self.data }
}

struct Subtype1 {}
impl Extend for Subtype1 { type Extra = String; }
impl Basetype<Subtype1> {
    fn get_string(&self) -> String { self.ext.clone() }
}

struct Subtype2 {}
impl Extend for Subtype2 { type Extra = usize; }

#[test]
fn main() {
    let a:Basetype<Subtype1> = Basetype{ data: 2, ext: String::from("yes") };
    println!("base data: {}",a.get_data());
    println!("ext data: {}",a.get_string());
}

