use uuid::Uuid;

pub struct User{
    uuid: Uuid,
    id: i32,
    login: String,
    pass: String,
    email: String,
    phone: String,
    name: String,
    code: String,
    description: String,
    notvalid: bool
}

impl User {
    pub fn new_user(login_: String, pass_: String, email_: String, phone_:String)-> User{
        User {
            uuid: Uuid::new_v4(),
            id : get_id(),
            login : login_,
            pass : pass_,
            email: email_,
            phone: phone_,
            name: "".to_string(),
            code: "".to_string(),
            description: "".to_string(),
            notvalid : true,
        }
    }
}
impl User {
    pub fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

fn get_id() -> i32{
    55
}