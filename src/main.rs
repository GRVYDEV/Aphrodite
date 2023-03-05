use crate::lexer::tokenize;
use easy_repl::{command, Command, CommandStatus, Repl};
use std::io;

mod lexer;
mod parser;
#[macro_use]
mod token;

fn main() -> io::Result<()> {
    println!("Aphrodite REPL v0.0.1");
    let mut repl = Repl::builder()
        .add(
            "t",
            Command {
                description: "tokenize".into(),
                args_info: vec!["code".into()],
                handler: Box::new(|args| {
                    let mut extended: Vec<&str> = Vec::new();
                    extended.extend(args);
                    let input = extended.join(" ");
                    match tokenize(&input) {
                        Ok(tkns) => {
                            println!("{:?}", tkns);
                        }
                        Err(e) => println!("Error tokenizing input {:?}", e),
                    };
                    Ok(CommandStatus::Done)
                }),
            },
        )
        .build()
        .expect("failed to create repl");

    repl.run().expect("whoops");
    Ok(())
}
