//! The input factory determines the source of input
//! input is assumed to mean "cargo test output" since it is ready by copper
//! how that input is received is determined by environment settings and command line
//! overrides
//!
//! there's two sources:
//!    STDIN  (default, use like 'cargo test | copper')
//!    a file (use like 'cargo test > test.txt && copper -f test.txt')

use input::from_file::FileReader;
use input::from_stdin::StdReader;
use input::input_trait::InputTrait;
use utils::environment::Environment;

/**
    InputTypes enums becomes a "placeholder" (so to speak) of the concrete implementation of
    InputTrait which handles the actual input reader.   This allows getting around "known size"
    requirements rust has on return types.

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations
*/
pub enum InputTypes {
    StdIn(StdReader),
    File(FileReader),
}

/**
    Think of this InputTrait implementation as something along the lines of an abstract class pointer
    or a v-table pointer in an OO language.

    The factory returns one of the enums along  with the associated implementation type allocated within the enum itself.
    Since consumers have the enum, the InputTrait implementation on the enum allows the consumer to work with the
    enum as a "class pointer" and not have to know about the actual implementation details; thereby making the
    consumer decoupled from the implementation.

    The concrete implementation are in their respective files.
*/
impl InputTrait for InputTypes {
    fn read_all(&self) -> Vec<String> {
        match *self {
            InputTypes::StdIn(ref reader) => return reader.read_all(),
            InputTypes::File(ref reader) => return reader.read_all(),
        }
    }
}

/**
*/
pub struct InputFactory {}

impl InputFactory {
    pub fn get(env : &Environment) -> InputTypes {
        let mut name: String = "stdin".to_string();

        if true == env.read_from_file {
            name = "file".to_string();
        }

        match name.as_ref() {
            "stdin" => InputTypes::StdIn(StdReader {} ),
            "file" => InputTypes::File(FileReader { file_name: env.input_file_name.to_string() }),
            _ => {
                eprintln!("Error in OutputFactory processing get for name of '{}'", name);
                panic!("Error in OutputFactory processing get request")
            },
        }
    }
}