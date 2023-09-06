use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{self, Device, Room};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn list_rooms(conn: &mut SqliteConnection) -> Result<Option<Vec<models::Room>>, DbError> {
    use crate::schema::rooms::dsl::*;

    let result = rooms.select(Room::as_select()).load(conn).optional()?;

    Ok(result)
}

pub fn add_devices(
    conn: &mut SqliteConnection,
    r_id: &str,
    device_names: Vec<String>,
) -> Result<Vec<models::Device>, DbError> {
    use crate::schema::devices::dsl::*;

    let new_devices = device_names
        .iter()
        .map(|d| models::Device {
            id: Uuid::new_v4().to_string(),
            name: d.to_string(),
            room_id: r_id.to_string(),
        })
        .collect::<Vec<models::Device>>();

    diesel::insert_into(devices)
        .values(&new_devices)
        .execute(conn)?;

    Ok(new_devices)
}

pub fn add_room_with_devices(
    conn: &mut SqliteConnection,
    room_name: &str,
    device_names: Vec<String>,
) -> Result<(Room, Vec<Device>), DbError> {
    conn.transaction(|connection| {
        let new_room = models::Room {
            id: Uuid::new_v4().to_string(),
            name: room_name.to_owned(),
        };
        let new_devices = device_names
            .iter()
            .map(|d| models::Device {
                id: Uuid::new_v4().to_string(),
                name: d.to_string(),
                room_id: new_room.id.to_string(),
            })
            .collect::<Vec<models::Device>>();

        {
            use crate::schema::rooms::dsl::*;
            diesel::insert_into(rooms)
                .values(&new_room)
                .execute(connection)?;
        }
        if !new_devices.is_empty() {
            use crate::schema::devices::dsl::*;
            diesel::insert_into(devices)
                .values(&new_devices)
                .execute(connection)?;
        }

        Ok((new_room, new_devices))
    })
}

pub fn del_room_with_devices(conn: &mut SqliteConnection, r_id: &str) -> Result<(), DbError> {
    conn.transaction(|connection| {
        {
            use crate::schema::rooms::dsl::*;
            diesel::delete(rooms)
                .filter(id.eq(r_id))
                .execute(connection)?;
        }
        {
            use crate::schema::devices::dsl::*;
            diesel::delete(devices)
                .filter(room_id.eq(r_id))
                .execute(connection)?;
        }

        Ok(())
    })
}

pub fn list_devices(
    conn: &mut SqliteConnection,
    r_id: &str,
) -> Result<Vec<models::Device>, DbError> {
    use crate::schema::devices::dsl::*;

    let result = devices
        .filter(room_id.eq(r_id))
        .select(Device::as_select())
        .load(conn)?;

    Ok(result)
}

pub fn del_device(conn: &mut SqliteConnection, d_id: &str) -> Result<(), DbError> {
    use crate::schema::devices::dsl::*;

    diesel::delete(devices).filter(id.eq(d_id)).execute(conn)?;

    Ok(())
}
