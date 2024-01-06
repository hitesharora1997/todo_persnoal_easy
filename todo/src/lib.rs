use std::{os::unix::process, process::exit};

#[warn(dead_code)]
#[derive(Debug)]
pub struct Task {
    task_id: u32,
    task: String,
    done_status: bool,
}

fn add_new_task(t: &mut Vec<Task>, n_task: &str) {
    let id_no: u64 = (t.len() + 1).try_into().unwrap();
    dbg!(id_no);
}

fn remove_task(t: &mut Vec<Task>, n_task: u64) {
    unimplemented!()
}

fn display_help() {
    unimplemented!()
}

fn display_todo(t: &mut Vec<Task>) {
    if t.is_empty() {
        println!("empty list");
    } else {
        println!("display todo {:?}", t);
    }
}

fn parse_arg(arg: Vec<&str>, t: &mut Vec<Task>) {
    dbg!(t.len());
    let command = arg[0];

    match command {
        "add" => {
            if let Some(val) = arg.get(1) {
                let new_task = *val;
                // Adding the todo list with the new task
                add_new_task(t, new_task);
                display_todo(t);
                println!("new {}", new_task);
            } else {
                println!("please provide a new name to the task");
            }
        }

        "show" => display_todo(t),

        "delete" => match arg[1].parse::<u64>() {
            Ok(value) => {
                remove_task(t, value);
            }
            Err(message) => eprintln!("Unable to delete the task {}", message.to_string()),
        },

        "exit" => exit(0),

        "help" | _ => {
            display_help();
            println!("in _ empty command")
        }
    };
}

pub fn run(arg: Vec<&str>, t: &mut Vec<Task>) {
    parse_arg(arg, t);
}
