use simple_repl::repl;

fn main() {
    println!("REPL Test:");
    println!();

    let _ = repl(eval);
}

fn eval(input: &str) -> Option<()> {
    println!("{input}");
    println!();
    Some(())
}
