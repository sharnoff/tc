mod ast;
mod db;
mod error;
mod token_tree;
mod tokens;
mod types;
mod utils;

use db::Files;

fn main() {
    let mut files = Files::new();

    files.reserve("test_input.tc");

    let test_input_file = files.file("test_input.tc");
    let token_tree = test_input_file.get_token_tree(|err| panic!("Failed to open file: {:?}", err));

    println!("{:?}", std::ops::Deref::deref(&token_tree));

    drop(token_tree);
    drop(test_input_file);

    let num_errors = files.print_errors();
    if num_errors != 0 {
        let num_err_str = match num_errors {
            1 => "a previous error".into(),
            n => format!("{} previous errors", n),
        };

        eprintln!(
            "{}: Failed due to {}",
            error::ERR_COLOR.paint("error"),
            num_err_str
        );
    }
}
