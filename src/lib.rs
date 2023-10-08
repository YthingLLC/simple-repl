use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub enum ReplError {
    ReadlineError,
    EvaluatorError,
}

impl std::convert::From<ReadlineError> for ReplError {
    fn from(_: ReadlineError) -> Self {
        ReplError::ReadlineError
    }
}

pub fn repl(evaluator: impl Fn(&str) -> Option<()>) -> std::result::Result<(), ReplError> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let input = rl.readline("> ")?;
        let input_str = input.as_str();

        rl.add_history_entry(input_str)?;

        match evaluator(input_str) {
            Some(_) => continue,
            None => break Err(ReplError::EvaluatorError),
        }
    }
}
