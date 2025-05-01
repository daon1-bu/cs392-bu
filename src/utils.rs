pub type Ident = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Copyable { Yes, No }

#[derive(Clone, Debug, PartialEq)]
pub enum Mutable { Yes, No }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Lifetime(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Lval {
    pub ident: Ident,
    pub derefs: usize,
}

impl Lval {
    pub fn new(name: &str, derefs: usize) -> Lval {
        Lval {
            ident: name.to_string(),
            derefs,
        }
    }

    pub fn var(name: &str) -> Lval {
        Lval::new(name, 0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Unit,
    Int(i32),
    Lval(Lval, bool),
    Box(Box<Expr>),
    Borrow(Lval, bool),
    Block(Vec<Stmt>, Box<Expr>, Lifetime),
    AssertEq(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign(Lval, Expr),
    LetMut(Ident, Expr),
    Expr(Expr),
}
