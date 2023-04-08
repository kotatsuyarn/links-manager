mod sqlite;
use sqlite::{db, list_group, list, task};
pub use list_group::{ListGroup};
pub use list::{List};
pub use task::{Task};

pub fn init_table_if_not_exists() {
  let tables = tables();

  if !tables.contains(&ListGroup::table_name()) {
    ListGroup::init_table_if_not_exists();
  }

  if !tables.contains(&List::table_name()) {
    List::init_table_if_not_exists();
  }

  if !tables.contains(&Task::table_name()) {
    Task::init_table_if_not_exists();
  }
}

pub fn tables() -> Vec<String> {
  let mut result: Vec<String> = Vec::new();

  db::connection(|conn| {
    let mut stmt = conn.prepare("
      select name
        from sqlite_schema
       where type = 'table' and name not like 'sqlite_%';
    ").unwrap();

    let rows = stmt.query_map([], |row| {
      let name: String = row.get(0).unwrap();
      return Ok(name);
    }).unwrap();

    for row in rows {
      result.push(row.unwrap());
    }
  });

  return result;
}