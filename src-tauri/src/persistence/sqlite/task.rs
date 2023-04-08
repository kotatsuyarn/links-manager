use uuid::Uuid;
use super::db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
  pub uuid: String,
  pub title: String,
  pub description: String,
  pub url: String,
  pub position: u32,
  pub list_uuid: String,
}

impl Task {
  pub fn table_name() -> String {
    return String::from("tasks");
  }

  pub fn init_table_if_not_exists() {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        CREATE TABLE IF NOT EXISTS tasks (
          uuid TEXT PRIMARY KEY,
          title TEXT NOT NULL,
          description TEXT NOT NULL,
          url TEXT NOT NULL,
          position INTEGER NOT NULL,
          list_uuid TEXT NOT NULL,
          FOREIGN KEY (list_uuid) 
            REFERENCES lists(uuid)
        );
      ").unwrap();
      stmt.execute([]).unwrap();
    });
  }

  pub fn new(
    uuid: String,
    title: String,
    description: String,
    url: String,
    position: u32,
    list_uuid: String
  ) -> Task {
    return Task {
      uuid,
      title,
      description,
      url,
      position,
      list_uuid,
    };
  }

  pub fn all() -> Vec<Task> {
    let mut result: Vec<Task> = Vec::new();

    db::connection(|conn| {
      let mut stmt = conn.prepare("
        SELECT uuid, title, description, url, position, list_uuid
          FROM tasks
         ORDER BY position;
      ").unwrap();
  
      let rows = stmt.query_map([], |row| {
        return Ok(
          Task::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
            row.get(4).unwrap(),
            row.get(5).unwrap()
          )
        );
      }).unwrap();
  
      for row in rows {
        result.push(row.unwrap());
      }
    });

    return result;
  }

  pub fn all_belong_to_list(list_uuid: String) -> Vec<Task> {
    let mut result: Vec<Task> = Vec::new();

    db::connection(|conn| {
      let mut stmt = conn.prepare("
        SELECT uuid, title, description, url, position, list_uuid
          FROM tasks
         WHERE list_uuid = ?1
         ORDER BY position;
      ").unwrap();
  
      let rows = stmt.query_map([list_uuid], |row| {
        return Ok(
          Task::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
            row.get(4).unwrap(),
            row.get(5).unwrap()
          )
        );
      }).unwrap();
  
      for row in rows {
        result.push(row.unwrap());
      }
    });

    return result;
  }

  pub fn create(
    title: String,
    description: String,
    url: String,
    position: u32,
    list_uuid: String
  ) -> Task {
    let task = Task::new(
      Uuid::new_v4().hyphenated().to_string(),
      title,
      description,
      url,
      position,
      list_uuid
    );

    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        INSERT INTO tasks (uuid, title, description, url, position, list_uuid) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);
      ").unwrap();

      stmt.execute(
        rusqlite::params![
          task.uuid,
          task.title,
          task.description,
          task.url,
          task.position,
          task.list_uuid,
        ]
      ).unwrap();
    });

    return task;
  }

  pub fn update(tasks: Vec<Task>) {
    db::transaction(|tx| {
      for task in tasks {
        let mut stmt = tx.prepare("
          UPDATE tasks
            SET title = ?2,
                description = ?3,
                url = ?4,
                position = ?5,
                list_uuid = ?6
          WHERE uuid = ?1;
        ").unwrap();

        stmt.execute(
          rusqlite::params![
            task.uuid,
            task.title,
            task.description,
            task.url,
            task.position,
            task.list_uuid
          ]
        ).unwrap();
      }
    });
  }

  pub fn delete(uuid: String) {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        DELETE FROM tasks
         WHERE uuid = ?1;
      ").unwrap();

      stmt.execute([uuid]).unwrap();
    });
  }
}