use actix_web::{
    web::{self, Data},
    Responder,
};
use pr0t0n_orch_db::{
    get_conn,
    models::{GetGroupRequest, SystemRepr},
    PgPool,
};

use crate::Error;

pub async fn upload(
    mut system: web::Json<SystemRepr>,
    pool: Data<PgPool>,
) -> Result<impl Responder, Error> {
    println!("Upload!");
    println!("System: {:#?}", system);
    let conn = get_conn(&pool)?;
    let res = system.sync_db(&conn);
    if let Err(e) = res {
        println!("Error: {:#?}", e);
    }
    Ok("Done")
}

pub async fn download(
    get_group_req: web::Json<GetGroupRequest>,
    pool: Data<PgPool>,
) -> Result<impl Responder, Error> {
    println!("Download!");
    let conn = get_conn(&pool)?;
    let system_repr = SystemRepr::get_group(&conn, get_group_req.asset_group_id)?;
    Ok(web::Json(system_repr))
}
