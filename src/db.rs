use rusqlite::{Connection, Result};

use crate::models::Task;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(path)?,
        })
    }

    pub fn setup_db(&self) -> Result<()> {
        self.conn.execute(
            "
                CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    description TEXT,
                    checked BOOLEAN NOT NULL CHECK (checked IN (0,1)) DEFAULT 0
                )
            ",
            [],
        )?;

        Ok(())
    }

    pub fn add_task(&self, description: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO tasks (description) VALUES (?1)", [description])?;
        Ok(())
    }

    pub fn check_task(&self, id: u16) -> Result<()> {
        self.conn
            .execute("UPDATE tasks SET checked = 1 - checked WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn fetch_tasks(&self) -> Result<Vec<Task>> {
        let mut cmd = self.conn.prepare("SELECT id, description, checked FROM tasks")?;

        let task_iter = cmd.query_map([], |row|{
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                checked: row.get(2)?
            })
        })?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            tasks.push(task_result?);
        }
        Ok(tasks)
    }

    pub fn remove_task(&self, id: u16) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }
}
