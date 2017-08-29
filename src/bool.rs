use prim::*;

pub trait Bool {}
impl Prim for Bool {}

pub struct True;
impl Bool for True {}

pub struct False;
impl Bool for False {}

pub trait Or<A:Bool,B:Bool> {type F:Bool;}
impl Or<True,True> for Bool {type F=True;}
impl Or<True,False> for Bool {type F=True;}
impl Or<False,True> for Bool {type F=True;}
impl Or<False,False> for Bool {type F=False;}
