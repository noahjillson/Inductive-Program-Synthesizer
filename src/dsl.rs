/*
 * The Domain Specific Language (DSL) used by the synthesizer
 */

enum Value {
    Param,
    Int(i32),
}

pub enum Expression<'a> {
    Unfilled,
    Terminator(Value),
    Add(&'a Expression<'a>, &'a Expression<'a>),
    Mult(&'a Expression<'a>, &'a Expression<'a>),
}

pub struct Queue<'a>(Vec<Expression<'a>>);

impl Queue<'_> {
    pub fn new() -> Self {
        Queue(Vec::new())
    }
}
