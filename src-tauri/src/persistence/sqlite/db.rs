use rusqlite::{Connection, Transaction, TransactionBehavior};

pub fn connection<F: FnOnce(&mut Connection) -> ()>(func: F) {
  let mut conn = Connection::open("../data/db.sqlite3").unwrap();
  func(&mut conn);
  conn.close().unwrap();
}

pub fn transaction<F: FnOnce(&Transaction) -> ()>(func: F) {
  connection(|conn| {
    let tx = Transaction::new(conn, TransactionBehavior::Deferred).unwrap();
    func(&tx);
    tx.commit().unwrap();
  });
}