use simple_repl::{repl, EvalResult, ReplResult, ReplError};

fn main() {
    println!("REPL Test:");
    println!();

    let res = repl(eval);

    if let Ok(ReplResult::Passthrough(value)) = res {
        println!("REPL Returned: {value}");
    }
    if let Err(ReplError::EvaluatorError(value)) = res {
        println!("{value:?}");
    }
}

#[derive(Debug)]
enum EvalError{
    IdkMan,
    Message(String),
}

fn eval(input: &str) -> Result<EvalResult, EvalError> {
    match input {
        "quit" => Ok(EvalResult::ExitRepl),
        "pass1" => Ok(EvalResult::Passthrough(1)),
        "error1" => Err(EvalError::Message("Error".to_string())),
        "error2" => Err(EvalError::IdkMan),
        x => {
            println!("{x}");
            println!();
            Ok(EvalResult::Continue)
        }
    }
}
