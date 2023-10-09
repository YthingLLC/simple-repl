use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

#[derive(Debug)]
pub enum ReplError<E>{
    ReadlineError,
    EvaluatorError(E),
}

pub enum EvalResult<T> {
    Continue,
    ExitRepl,
    Passthrough(T),
}

pub enum ReplResult<T> {
    ExitRepl,
    Passthrough(T)
}

impl <E> std::convert::From<ReadlineError> for ReplError<E> {
    fn from(_: ReadlineError) -> Self {
        ReplError::ReadlineError
    }
}

pub fn repl<T, E> (evaluator: impl Fn(&str) -> Result<EvalResult<T>, E>) -> std::result::Result<ReplResult<T>, ReplError<E>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let input = rl.readline("> ")?;
        let input_str = input.as_str();

        rl.add_history_entry(input_str)?;

        match evaluator(input_str) {
            Ok(EvalResult::Continue) => continue,
            Ok(EvalResult::ExitRepl) => break Ok(ReplResult::ExitRepl),
            Ok(EvalResult::Passthrough(x)) => break Ok(ReplResult::Passthrough(x)),
            Err(e) => break Err(ReplError::EvaluatorError(e)),
        }
    }
}
