use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub enum ReplError<E> {
    ReadlineError,
    EvaluatorError(E),
}

impl <E> std::convert::From<ReadlineError> for ReplError<E> {
    fn from(_: ReadlineError) -> Self {
        ReplError::ReadlineError
    }
}

pub fn repl<E> (evaluator: impl Fn(&str) -> Result<(), E>) -> std::result::Result<(), ReplError<E>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let input = rl.readline("> ")?;
        let input_str = input.as_str();

        rl.add_history_entry(input_str)?;

        match evaluator(input_str) {
            Ok(_) => continue,
            Err(e) => break Err(ReplError::EvaluatorError(e)),
        }
    }
}
