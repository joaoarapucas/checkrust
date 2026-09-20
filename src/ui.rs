use crate::models::Task;
use colored::*;

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "no task to show.".yellow());
        return;
    }

    println!("{}", "--- TASKS ---".bold());

    for task in tasks {
        let status = if task.checked { "[x]" } else { "[ ]" };

        let line = format!("{} {} {}", task.id, status, task.description);

        if task.checked {
            println!("{}", line.green().strikethrough());
        } else {
            println!("{}", line.red());
        }
    }
}
