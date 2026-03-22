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

enum Brace {
    Curly,
    Round,
}

fn find_closing_brace(linked_chars: &LinkedChars, opening_brace_idx: usize, brace: Brace) -> usize {
    let mut number_opened = 1;
    let next_idx = opening_brace_idx;
    for node in linked_chars.iter_from(opening_brace_idx) {
        match linked_chars.get(next_idx).c {
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
            return next_idx;
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
    parent: &'a Interpreter, //
    registers: Vec<String>,
}
