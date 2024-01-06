#[warn(dead_code)]
#[derive(Debug)]
pub struct Task {
    task_Id: u32,
    task: String,
    done_Status: bool,
}

fn add_new_task(t: &mut Vec<Task>, nTask: &str) {
    unimplemented!()
}

fn display_todo(t: &mut Vec<Task>) {
    println!("{:?}", t);
}

fn parse_arg(arg: Vec<&str>, t: &mut Vec<Task>) {
    let command = arg[0];

    match command {
        "add" => {
            if let Some(val) = arg.get(1) {
                let new_task = *val;
                // Adding the todo list with the new task
                add_new_task(t, new_task);
                display_todo(t);
            }
        }
        _ => println!("empty command"),
    };
}

pub fn run(arg: Vec<&str>, t: &mut Vec<Task>) {
    parse_arg(arg, t);
}
