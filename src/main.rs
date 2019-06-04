pub mod instruction;
pub mod vm;
pub mod repl;
pub mod lexer;


fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
