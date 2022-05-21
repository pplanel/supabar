use rustyline::hint::Hinter;
use std::collections::HashSet;

use rustyline::hint::Hint;
use rustyline::{error::ReadlineError, Editor};
use rustyline::{
    Cmd, ConditionalEventHandler, Context, Event, EventContext, EventHandler, KeyEvent, RepeatCount,
};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use supabar::{ClientCommand, ClientQuery, Handler, Response};

struct TabEventHandler;
impl ConditionalEventHandler for TabEventHandler {
    fn handle(&self, evt: &Event, _n: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        debug_assert_eq!(*evt, Event::from(KeyEvent::from('\t')));

        if ctx.has_hint() {
            Some(Cmd::CompleteHint)
        } else {
            None // default complete
        }
    }
}

#[derive(Completer, Helper, Validator, Highlighter)]
struct DIYHinter {
    // It's simple example of rustyline, for more efficient, please use ** radix trie **
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for DIYHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints.iter().find_map(|hint| {
            // expect hint after word complete, like redis cli, add condition:
            // line.ends_with(" ")
            if hint.display.starts_with(line) {
                Some(hint.suffix(pos))
            } else {
                None
            }
        })
    }
}

pub async fn run_cli(core_handler: &Handler) {
    let h = DIYHinter {
        hints: get_app_hints(core_handler).await,
    };
    let mut rl: Editor<DIYHinter> = Editor::new();
    rl.set_helper(Some(h));
    rl.bind_sequence(
        KeyEvent::from('\t'),
        EventHandler::Conditional(Box::new(TabEventHandler)),
    );
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match line.parse::<ClientCommand>() {
                Ok(command) => match core_handler.command(command).await {
                    Ok(_response) => {
                        println!("Command went througt")
                    }
                    Err(err) => println!("command error {:?}", err),
                },
                Err(err) => println!("command error {:?}", err),
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

async fn get_app_hints(core_handler: &Handler) -> HashSet<CommandHint> {
    match core_handler.query(ClientQuery::ListApplications).await {
        Ok(Response::ListApplications { applications }) => {
            applications
                .into_iter()
                .fold(HashSet::new(), |mut hs, app| {
                    hs.insert(CommandHint::new(&app.as_str(), app.as_str()));
                    hs
                })
        }
        _ => panic!("asd"),
    }
}
