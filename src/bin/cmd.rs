use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Result;
use rustyline::{error::ReadlineError, Editor};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputValidator {}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        use ValidationResult::{Invalid, Valid};
        let input = ctx.input();
        let result = if input.is_empty() {
            Invalid(None)
        } else {
            Valid(None)
        };
        Ok(result)
    }
}

fn main() {
    let validator = InputValidator {};
    let mut rl = Editor::new();
    rl.set_helper(Some(validator));
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match supabar::execute(&line) {
                Ok(msg) => println!("Message {}", msg),
                Err(err) => println!("Error {}", err),
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
