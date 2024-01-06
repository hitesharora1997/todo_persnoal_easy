#[warn(dead_code)]
pub struct Task {
    task_Id: u32,
    task: String,
    done_Status: bool,
}

fn parse_arg(arg: Vec<&str>, t: &mut Vec<Task>) {
    let command = arg[0];
    println!("{}", command);
}

pub fn run(arg: Vec<&str>, t: &mut Vec<Task>) {
    parse_arg(arg, t);
}
