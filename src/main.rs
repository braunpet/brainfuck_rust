mod brainfuck;
use crate::brainfuck::run;

fn main() {
    run("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.".to_string());
}
