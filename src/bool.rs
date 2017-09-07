use base::*;

pub struct Bool(());

pub struct True;
impl Type for True {type T=Bool;}

pub struct False;
impl Type for False {type T=Bool;}

pub struct Or;
impl Arrow2 for Or {type T1=Bool;type T2=Bool; type T3=Bool;}
impl Func2<True,True> for Or {type F=True;}
impl Func2<True,False> for Or {type F=True;}
impl Func2<False,True> for Or {type F=True;}
impl Func2<False,False> for Or {type F=False;}
