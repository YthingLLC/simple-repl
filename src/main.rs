//use the library
use simple_repl::{repl, EvalResult, ReplResult, ReplError};

fn main() {
    println!("REPL Test:");
    println!();

    //pass control to the repl loop, with the desired evaluator
    let res = repl(eval);

    //check the results from the repl for a passthrough value or error
    if let Ok(ReplResult::Passthrough(value)) = res {
        println!("REPL Returned: {value}");
    }
    if let Err(ReplError::EvaluatorError(value)) = res {
        println!("{value:?}");
    }
}

//define our error types, you can also just use a String if you don't want to define a type
#[derive(Debug)]
enum EvalError{
    IdkMan,
    Message(String),
}


//the evaluator is passed a single "line" to be evaluated
//this example shows a few different evaluation choices (i.e. an interactive CLI!)

//Any input that is not "quit", "pass1", "error1", or "error2" will be echo'd back to the user

//pass1 shows an example of passing a value back through to the caller of the repl
//error1 and error2 show different possible errors being returned to the caller of the repl

//evaluator must return Result<EvalResult<T>, EvalError<E>> and accept a &str

//if `Err(E)` or `EvalResult::ExitRepl` is returned, the repl loop ends and control is returned
//to the caller

//if EvalResult::Passthrough is returned, the type T must be provided

//if EvalResult::Continue is passed, control is returned to the repl loop for additional user input
fn eval(input: &str) -> Result<EvalResult<isize>, EvalError> {
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
