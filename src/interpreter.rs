use crate::linkes_chars::LinkedChars;

enum Task {
    Scope {
        content: String,
    },
    FunctionCall {
        function_name: String,
        input: String,
    },
    RegisterCall {
        level: usize,
        index: usize,
    },
    GetInput {
        prompt: String,
    },
    PrintOutput {
        content: String,
    },
}

struct Job {
    start: usize, // the star index of the stuff to be replaced
    end: usize,   // end index
    task: Task,
}

fn get_new_job(linked_chars: &LinkedChars, reader_idx: usize) -> Job {
    let mut chars_buffer = Vec::new(); // holds the read chars
    loop {
        match linked_chars.get(reader_idx).c {
            '(' => { // this is a function call. Find the closing brace
            }
            c => chars_buffer.push(c),
        }
    }
    unimplemented!()
}

#[derive(PartialEq)]
enum Brace {
    Curly,
    Round,
}

// returns the index to the node after the closing brace, or none if the closing brace is the last
// char in the linked_chars
// panics if there is no closing brace
fn find_closing_brace(linked_chars: &LinkedChars, opening_brace_idx: usize, brace: Brace) -> usize {
    let mut number_opened = 1;
    for (idx, node) in linked_chars.iter_with_start(opening_brace_idx) {
        match node.c {
            '{' => {
                if brace == Brace::Curly {
                    number_opened += 1
                }
            }
            '(' => {
                if brace == Brace::Round {
                    number_opened += 1
                }
            }
            '}' => {
                if brace == Brace::Curly {
                    number_opened -= 1
                }
            }
            ')' => {
                if brace == Brace::Round {
                    number_opened -= 1
                }
            }
            _ => (),
        }
        if number_opened == 0 {
            return idx;
        }
    }
    panic!("No closing brace found"); // TODO: add proper error handling
}

// an Interpreter gets passed a LinkedChars and is tasked to evaluate it until there are no further
// changes
// It will save regex matches into its own registers
// its children may use the contents of these registers by using the ^ operator on register calls
//
struct Interpreter<'a> {
    state: LinkedChars,
    parent: &'a Interpreter<'a>, //
    registers: Vec<String>,
}
