use base::*;

pub struct Bool;
impl Type for Bool {type T=Star;}

pub struct True;
impl Type for True {type T=Bool;}

pub struct False;
impl Type for False {type T=Bool;}

pub struct Or;
impl Type for Or {type T=Arrow2<Bool,Bool,Bool>;}
impl Func2<True,True> for Or {type F=True;}
impl Func2<True,False> for Or {type F=True;}
impl Func2<False,True> for Or {type F=True;}
impl Func2<False,False> for Or {type F=False;}
