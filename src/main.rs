use clap::Parser;
use rusqlite::Result;

use crate::cli::Cli;
use crate::db::Database;

mod cli;
mod db;
mod models;
mod ui;

fn main() -> Result<()> {
    println!("Hello, world!");

    let db = Database::new("checklist.db")?;
    db.setup_db()?;

    let args = Cli::parse();

    if let Some(description) = args.add {
        db.add_task(&description)?;
        println!("task added!");
    }
    if let Some(id) = args.check {
        db.check_task(id)?;
        println!("task {} checked!", id);
    }

    if let Some(id) = args.remove {
        db.remove_task(id)?;
        println!("task {} removed.", id);
    }

    if args.list {
        let tasks = db.fetch_tasks()?;
        ui::print_tasks(&tasks);
    }

    println!("finished!");

    Ok(())
}
