use crate::interpreter::*;
use crate::linked_chars::*;

fn evaluate_scope(scope: String, parent_interpreter: &Interpreter) -> LinkedChars {
    let lc = LinkedChars::from_iter(scope.chars());
    // first, find the input to the scope: { INPUT :
    // ignore whitespace after the { and before the :
    let mut input_buffer = Vec::new();
    for (i, node) in lc.enumerate_with_start(0) {
        match node.c {
            ':' => break,
            ' ' | '\t' | '\n' => (),
            c => input_buffer.push(c),
        }
    }
    // evaluate the input to resolve any function calls or scopes in it
    let input_lc = LinkedChars::from_iter(input_buffer);
    let mut input_interpreter = Interpreter {
        state: input_lc,
        registers: vec![],
        parent: Some(parent_interpreter),
        functions: parent_interpreter.functions.clone(),
    };
    unimplemented!()
}
