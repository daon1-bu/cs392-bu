use std::env;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use salt::parser::Parser;
use salt::eval;
use salt::types;
use salt::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = {
        let mut args = env::args();
        args.next();
        args.next().ok_or(Error::new(ErrorKind::NotFound, "usage: cargo run --bin interp <filename>"))?
    };

    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    let mut e = Parser::new(&contents[..])
        .parse()
        .map_err(|err| Error::new(ErrorKind::Other, format!("{:?}", err)))?;

        types::TypeContext::new()
        .type_expr(&mut e)
        .map_err(|err| Error::new(ErrorKind::Other, format!("{:?}", err)))?;

    eval::Context::default().eval_expr(&e, Lifetime::global());

    Ok(())
}
