extern crate todo;
use std::io::{stdout, Write};

use todo::Task;

fn runpromt(t: &mut Vec<Task>) {
    // Keep running in a roop like while
    loop {
        let mut stdout = stdout();

        println!("<");
        // Write output to the script before the program exists
        stdout.flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read line");

        let args: Vec<&str> = input.split_whitespace().collect();
        println!("{:?}", args);

        todo::run(args, t);
    }
}

fn main() {
    // Defining a new vector
    let mut todo: Vec<Task> = Vec::new();

    // Mutable Borrowing the todo vector
    runpromt(&mut todo);
}
