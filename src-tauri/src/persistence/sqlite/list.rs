use uuid::Uuid;
use super::db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct List {
  pub uuid: String,
  pub name: String,
  pub position: u32,
  pub list_group_uuid: Option<String>,
}

impl List {
  pub fn table_name() -> String {
    return String::from("lists");
  }

  pub fn init_table_if_not_exists() {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        CREATE TABLE IF NOT EXISTS lists (
          uuid TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          position INTEGER NOT NULL,
          list_group_uuid TEXT,
          FOREIGN KEY (list_group_uuid) 
            REFERENCES list_groups(uuid)
        );
      ").unwrap();
      stmt.execute([]).unwrap();
    });
  }

  pub fn new(
    uuid: String,
    name: String,
    position: u32,
    list_group_uuid: Option<String>
  ) -> List {
    return List {
      uuid,
      name,
      position,
      list_group_uuid,
    };
  }

  pub fn all() -> Vec<List> {
    let mut result: Vec<List> = Vec::new();

    db::connection(|conn| {
      let mut stmt = conn.prepare("
        SELECT uuid, name, position, list_group_uuid
          FROM lists
         ORDER BY position;
      ").unwrap();
  
      let rows = stmt.query_map([], |row| {
        return Ok(
          List::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap()
          )
        );
      }).unwrap();
  
      for row in rows {
        result.push(row.unwrap());
      }
    });

    return result;
  }

  pub fn create(name: String, position: u32, list_group_uuid: Option<String>) -> List {
    let list = List::new(
      Uuid::new_v4().hyphenated().to_string(),
      name,
      position,
      list_group_uuid
    );

    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        INSERT INTO lists (uuid, name, position, list_group_uuid) 
        VALUES (?1, ?2, ?3, ?4);
      ").unwrap();

      stmt.execute(
        rusqlite::params![
          list.uuid,
          list.name,
          list.position,
          list.list_group_uuid,
        ]
      ).unwrap();
    });

    return list;
  }

  pub fn update(lists: Vec<List>) {
    db::transaction(|tx| {
      for list in lists {
        let mut stmt = tx.prepare("
          UPDATE lists
            SET name = ?2,
                position = ?3,
                list_group_uuid = ?4
          WHERE uuid = ?1;
        ").unwrap();

        stmt.execute(
          rusqlite::params![
            list.uuid,
            list.name,
            list.position,
            list.list_group_uuid
          ]
        ).unwrap();
      }
    });
  }

  pub fn delete(uuid: String) {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        DELETE FROM lists
         WHERE uuid = ?1;
      ").unwrap();

      stmt.execute([uuid]).unwrap();
    });
  }
}
