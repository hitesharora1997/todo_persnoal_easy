use std::process::exit;
#[warn(unused_variables, dead_code)]
#[derive(Debug)]
pub struct Task {
    task_id: u32,
    task: String,
    done_status: bool,
}

fn add_new_task(_t: &mut Vec<Task>, n_task: &str) {
    // to convert the length in u64 we used try_into methods
    let id_no = (_t.len() + 1).try_into().unwrap();
    let task = Task {
        task: n_task.to_string(),
        task_id: id_no,
        done_status: false,
    };

    // Pushing new task to the vec of struct
    _t.push(task);

    println!("{} added to the todo list", n_task);
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
        return;
    }

    for item in t {
        println!(
            "{}: name: {}, done:{}",
            item.task_id, item.task, item.done_status
        );
    }
}

fn parse_arg(arg: Vec<&str>, t: &mut Vec<Task>) {
    let command = arg[0];

    match command {
        "add" => {
            if let Some(val) = arg.get(1) {
                // New tak is the one captured by arg.get
                let new_task = *val;
                // Adding the todo list with the new task
                add_new_task(t, new_task);
                display_todo(t);
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
