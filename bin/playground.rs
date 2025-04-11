
use salt::utils::*;
use salt::eval::Context;


fn main() {
    let mut ctx = Context {
        store: Default::default(),
    };

    let lifetime = Lifetime::global();

 
    let block_expr = Expr::Block(
        vec![
            Stmt::LetMut(
                "x".to_string(),
                Expr::Int(1),
            ),
            Stmt::LetMut(
                "y".to_string(),
                Expr::Borrow(
                    Lval {
                        ident: "x".to_string(),
                        derefs: 0,
                    },
                    false,
                ),
            ),
        ],





        
        Box::new(Expr::Lval(
            Lval {
                ident: "x".to_string(),
                derefs: 0,
            },
            true, 
        )),
        lifetime.clone(),
    );


    let result = ctx.eval_expr(&block_expr, lifetime);

    println!("Final value: {:?}", result);
}
