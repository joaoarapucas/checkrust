use std::fs;
use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;
use rusqlite::Result;

use crate::cli::Cli;
use crate::db::Database;

mod cli;
mod db;
mod models;
mod ui;

fn db_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "checkrust", "checkrust")
        .expect("could not determine user's data directory");
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir).expect("failed to create the data directory");
    data_dir.join("checklist.db")
}

fn main() -> Result<()> {
    let db = Database::new(db_path().to_str().expect("invalid database path"))?;
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

    Ok(())
}
