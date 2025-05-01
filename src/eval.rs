use std::collections::HashMap;
use crate::utils::*;

// Owned flag (true = we own it, false = borrowed)
pub type Owned = bool;
type Location = Ident;
type Pvalue = Option<Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(i32),
    Ref(Location, Owned),
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub value: Pvalue,
    pub lifetime: Lifetime,
}

#[derive(Clone, Debug, Default)]
pub struct Store(pub HashMap<Location, Slot>);

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub store: Store,
}

impl Store {
    pub fn locate<'a>(&self, lval: &'a Lval) -> &'a Location {
        &lval.ident
    }

    pub fn read(&self, lval: &Lval) -> &Slot {
        let loc = self.locate(lval);
        self.0.get(loc).expect("read: location not found")
    }

    pub fn write(&mut self, lval: &Lval, new_val: Pvalue) -> Pvalue {
        let loc = self.locate(lval).clone();
        let slot = self.0.get(&loc).expect("write: location not found");
        let old_val = slot.value.clone();
        let lifetime = slot.lifetime.clone();

        self.0.insert(loc, Slot {
            value: new_val,
            lifetime,
        });

        old_val
    }

    pub fn drop(&mut self, to_remove: Vec<Pvalue>) {
        for val in to_remove {
            if let Some(Value::Ref(loc, true)) = val {
                self.0.remove(&loc);
            }
        }
    }
}

impl Lifetime {
    pub fn global() -> Lifetime {
        Lifetime(0)
    }
}

impl Context {
    pub fn eval_expr(&mut self, expr: &Expr, l: Lifetime) -> Value {
        match expr {
            Expr::Unit => Value::Unit,
            Expr::Int(n) => Value::Int(*n),

            Expr::Lval(lval, copyable) => {
                let slot = self.store.read(lval);
                let v = slot.value.clone().unwrap();

                if !*copyable {
                    self.store.write(lval, None);
                }

                v
            }

            Expr::Box(e) => {
                let v = self.eval_expr(e, l.clone());
                let fresh_id = format!("loc_{}", self.store.0.len());
                self.store.0.insert(fresh_id.clone(), Slot {
                    value: Some(v),
                    lifetime: Lifetime::global(),
                });
                Value::Ref(fresh_id, true)
            }

            Expr::Borrow(lval, _is_mut) => {
                let loc = self.store.locate(lval).clone();
                Value::Ref(loc, false)
            }

            Expr::Block(stmts, final_expr, block_lifetime) => {
                for stmt in stmts {
                    self.eval_stmt(stmt, block_lifetime.clone());
                }

                let result = self.eval_expr(final_expr, block_lifetime.clone());

                let to_drop: Vec<_> = self.store.0
                    .iter()
                    .filter(|(_, slot)| slot.lifetime == *block_lifetime)
                    .map(|(_, slot)| slot.value.clone())
                    .collect();

                self.store.drop(to_drop);

                result
            }
            Expr::AssertEq(left, right) => {
                let v1 = self.eval_expr(left, l.clone());
                let v2 = self.eval_expr(right, l);
                match (v1, v2) {
                    (Value::Int(a), Value::Int(b)) => assert_eq!(a, b),
                    _ => panic!("assert_eq! only supports integers"),
                }
                Value::Unit
            }
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt, l: Lifetime) {
        match stmt {
            Stmt::Assign(lval, expr) => {
                let val = self.eval_expr(expr, l.clone());
                self.store.write(lval, Some(val));
            }

            Stmt::LetMut(ident, expr) => {
                let val = self.eval_expr(expr, l.clone());
                self.store.0.insert(ident.clone(), Slot {
                    value: Some(val),
                    lifetime: l,
                });
            }

            Stmt::Expr(expr) => {
                self.eval_expr(expr, l);
            }
        }
    }
}
