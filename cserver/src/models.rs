use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Debug, Deserialize)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Room {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AddRoom {
    pub name: String,
    pub devices: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddedRoom {
    pub room: Room,
    pub devices: Vec<Device>,
}

#[derive(Deserialize)]
pub struct Id {
    pub id: String,
}

#[derive(Queryable, Selectable, Insertable, Associations, Serialize, Debug, Deserialize)]
#[diesel(table_name = crate::schema::devices)]
#[diesel(belongs_to(Room))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Device {
    pub id: String,
    pub name: String,
    pub room_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AddDevice {
    pub room_id: String,
    pub device_name: String,
}
