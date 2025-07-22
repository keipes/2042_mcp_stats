//! Database management modules
pub mod manager;
mod table;
// #[derive(Debug, Clone, Archive, RkyvSerialize, RkyvDeserialize)]
// pub struct SimpleRecord {
//     pub name: String,
// }

// pub struct BasicDb {
//     env: Environment,
//     db: lmdb_rs::DbHandle,
// }

// impl BasicDb {
//     pub fn open(path: &str) -> Self {
//         let env = Environment::new().open(path, 0o600).unwrap();
//         let db = env.get_default_db(DbFlags::empty()).unwrap();
//         let txn = env.new_transaction().unwrap();
//         // write something
//         let record = SimpleRecord {
//             name: "Example".to_string(),
//         };
//         // let bytes = to_bytes(&record).unwrap();
//         let real_db = txn.bind(&db);
//         real_db.insert(&1, &record.name);

//         txn.commit().unwrap();
//         Self { env, db }
//     }

// }
