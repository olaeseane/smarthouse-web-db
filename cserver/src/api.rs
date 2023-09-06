use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Result};

use crate::{
    actions,
    db::DbPool,
    errors::WebServerError,
    models::{self, AddDevice, AddedRoom},
};

#[get("/rooms")]
async fn list_rooms(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        actions::list_rooms(&mut conn)
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(web::Json(rooms))
}

#[post("/room")]
async fn add_room(
    pool: web::Data<DbPool>,
    params: web::Json<models::AddRoom>,
) -> Result<impl Responder> {
    let new_room_with_devices = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_room_with_devices(&mut conn, &params.name, params.devices.clone())
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(web::Json(AddedRoom {
        room: new_room_with_devices.0,
        devices: new_room_with_devices.1,
    }))
}

#[delete("/room/{id}")]
async fn del_room(pool: web::Data<DbPool>, path: web::Path<models::Id>) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get()?;
        actions::del_room_with_devices(&mut conn, &path.id)
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/devices/room/{id}")]
async fn list_devices(
    pool: web::Data<DbPool>,
    path: web::Path<models::Id>,
) -> Result<HttpResponse> {
    let devices = web::block(move || {
        let mut conn = pool.get()?;
        actions::list_devices(&mut conn, &path.id)
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(HttpResponse::Ok().json(web::Json(devices)))
    // Ok(web::Json(devices))
}

#[post("/device")]
async fn add_device(
    pool: web::Data<DbPool>,
    params: web::Json<AddDevice>,
) -> Result<impl Responder> {
    let device = params.into_inner();
    let new_device = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_devices(&mut conn, &device.room_id, vec![device.device_name])
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(web::Json(new_device))
}

#[delete("/device/{id}")]
async fn del_device(
    pool: web::Data<DbPool>,
    path: web::Path<models::Id>,
) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get()?;
        actions::del_device(&mut conn, &path.id)
    })
    .await?
    .map_err(|e| WebServerError {
        msg: format!("{e}"),
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    })?;

    Ok(HttpResponse::Ok().finish())
}
