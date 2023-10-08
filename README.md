# simple-repl

This is a *very* simple base for interactive repl / CLI applications.

The purpose of this is, primarily, to prevent me from having to rewrite this 'boilerplate' in every small CLI 
application I have been working on. 

Usage:

```rust
//use the library
use simple_repl::repl;

fn main() {
    println!("REPL Test:");
    println!();
    
    //pass control to the repl loop, with the desired evaluator
    let _ = repl(eval);
}

//the evaluator is passed a single "line" to be evaluated
//this example simply echos the user input
//evaluator must return Option<()> and accept a &str
//if `None` is returned, the repl loop breaks and returns to the function which called it
fn eval(input: &str) -> Option<()> {
    println!("{input}");
    println!();
    Some(())
}
```

Try it out! The above example is in main.rs, and can be run on your PC with `cargo run`!




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