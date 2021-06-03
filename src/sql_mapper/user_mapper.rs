use postgres::{Column, Row};
use crate::models::User;

pub fn user_mapper(row: &Row)->User{
    User{
        uuid: row.get(0),
        login: row.get(1),
        pass: row.get(2),
        email: row.get(3),
        phone: row.get(4),
        id: row.get(5),
        code: row.get(6),
        name: row.get(7),
        description: row.get(8),
        notvalid: row.get(9)
    }
}