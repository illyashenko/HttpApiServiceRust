//use postgres::{Client, NoTls, Column};
use postgres::{Client, NoTls, Column};

pub struct Context{
   pub client: Client
}

pub struct ContextBuilder {}

impl ContextBuilder{
   pub fn new() -> Context {
        Context{ client: Client::connect("postgresql://postgres:17101030@localhost/STORE", NoTls).unwrap()}
    }
}

/*pub fn context() {
  /*  for row in client.query("SELECT * FROM guids_users", &[]).unwrap() {
        let columns = row.columns();
        let tt = columns[4].type_().name();
        let tes = true;
        let x = if tes {
            row.get(5)
        } else { 0 };
        let guid: Uuid = row.get(0);
        let login: String = row.get(1);
        let name: String = row.get(3);
        let data: String = row.get(7);
        println!("Data:{:?} {} {} {:?} {:?}", x, login, name, data, guid);
    } */
}*/