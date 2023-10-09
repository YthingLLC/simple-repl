# simple-repl

This is a *very* simple base for interactive repl / CLI applications.

The purpose of this is, primarily, to prevent me from having to rewrite this 'boilerplate' in every small CLI 
application I have been working on. 

This uses the [rustyline](https://crates.io/crates/rustyline) crate to provide command history, line editing, etc.

Usage:

```rust
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

```

Try it out! The above example is in main.rs, and can be run on your PC with `cargo run`!

For an example of how I use this with another piece of software, check out my [rpn calculator](https://github.com/davidsenk/rpn) [main.rs](https://github.com/davidsenk/rpn/blob/devel/src/main.rs)!


I do not provide any guarantees, warranties, etc. This is licensed under the MIT license:

```
Copyright © 2023 Ything LLC 

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the “Software”), to deal in the Software without restriction, including without limitation the 
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO 
THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. 

IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, 
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.
```