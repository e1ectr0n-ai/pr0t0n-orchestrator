use actix_web::{
    web::{self, Data},
    HttpRequest, Responder,
};
use pr0t0n_orch_db::{models::Service, PgPool};
use serde::{Deserialize, Serialize};

pub async fn config_sync(
    state: web::Json<ServiceRepr>,
    pool: Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    Ok("Done")
}
