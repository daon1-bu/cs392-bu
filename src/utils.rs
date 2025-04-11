pub type Ident = String;
type Copyable = bool;  // can copy this (like int)
type Mutable = bool;   // true if it's &mut

#[derive(Clone, Debug, PartialEq)]
pub struct Lifetime(pub usize); // for tracking block lifetime

#[derive(Clone, Debug)]
pub struct Lval {
    pub ident: Ident,    // var name
    pub derefs: usize,   // how many * to go through
}

// stuff the language understands
#[derive(Clone, Debug)]
pub enum Expr {
    Unit,                          // nothing
    Int(i32),                      // just a number
    Lval(Lval, Copyable),          // var or *var
    Box(Box<Expr>),               // heap alloc
    Borrow(Lval, Mutable),         // & or &mut
    Block(Vec<Stmt>, Box<Expr>, Lifetime), // code block with lifetime
}

// stuff that does things
#[derive(Clone, Debug)]
pub enum Stmt {
    Assign(Lval, Expr),       // x = something
    LetMut(Ident, Expr),      // let mut x = something
    Expr(Expr),               // just run this
}
