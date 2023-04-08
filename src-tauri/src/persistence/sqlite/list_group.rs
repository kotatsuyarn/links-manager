use uuid::Uuid;
use super::db;

pub struct ListGroup {
  pub uuid: String,
  pub name: String,
  pub position: u32,
}

impl ListGroup {
  pub fn table_name() -> String {
    return String::from("list_groups");
  }

  pub fn init_table_if_not_exists() {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        CREATE TABLE IF NOT EXISTS list_groups (
          uuid TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          position INTEGER NOT NULL
        );
      ").unwrap();
      stmt.execute([]).unwrap();
    });
  }

  pub fn new(uuid: String, name: String, position: u32) -> ListGroup {
    return ListGroup { uuid, name, position };
  }

  pub fn all() -> Vec<ListGroup> {
    let mut result: Vec<ListGroup> = Vec::new();

    db::connection(|conn| {
      let mut stmt = conn.prepare("
        select uuid, name, position
          from list_groups;
      ").unwrap();
  
      let rows = stmt.query_map([], |row| {
        return Ok(
          ListGroup::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap()
          )
        );
      }).unwrap();
  
      for row in rows {
        result.push(row.unwrap());
      }
    });

    return result;
  }

  pub fn create(name: String, position: u32) -> ListGroup {
    let lg = ListGroup::new(
      Uuid::new_v4().hyphenated().to_string(),
      name,
      position
    );

    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        INSERT INTO list_groups (uuid, name, position) VALUES (?1, ?2, ?3);
      ").unwrap();

      stmt.execute([
        lg.uuid.to_owned(), lg.name.to_owned(), lg.position.to_string()
      ]).unwrap();
    });

    return lg;
  }

  pub fn read(uuid: String) -> Option<ListGroup> {
    let mut result: Option<ListGroup> = None;

    db::connection(|conn| {
      let mut stmt = conn.prepare("
        select uuid, name, position
          from list_groups
        where uuid = ?1;
      ").unwrap();

      let rows = stmt.query_map([uuid], |row| {
        return Ok(
          ListGroup::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap()
          )
        );
      }).unwrap();

      for row in rows {
        result = Some(row.unwrap());
        break;
      }
    });

    return result;
  }

  pub fn update(uuid: String, name: String, position: u32) {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        UPDATE list_groups
           SET name = ?2,
               position = ?3
         WHERE uuid = ?1;
      ").unwrap();

      stmt.execute([uuid, name, position.to_string()]).unwrap();
    });
  }

  pub fn delete(uuid: String) {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        DELETE FROM list_groups
         WHERE uuid = ?1;
      ").unwrap();

      stmt.execute([uuid]).unwrap();
    });
  }

  pub fn delete_all() {
    db::transaction(|tx| {
      let mut stmt = tx.prepare("
        DELETE FROM list_groups;
      ").unwrap();

      stmt.execute([]).unwrap();
    });
  }
}