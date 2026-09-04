//! HTTP surface for `/install` when `install/dist` exists on disk.

use actix_files::Files;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::install_env::{
    existing_prefix_needs_wipe, run_install_write, InstallEnvError, InstallWriteOptions,
};
use crate::install_fs::{
    install_artefacts_present, install_dir, shop_root, INSTALL_DIR_NAME, INSTALL_OFF_DIR_NAME,
};

/// Shared root path for install disk checks.
#[derive(Clone, Debug)]
pub struct InstallRoot(pub std::path::PathBuf);

#[derive(Debug, Serialize)]
struct StatusResponse {
    available: bool,
    wipe_required: bool,
    rename_after_success: String,
}

#[derive(Debug, Deserialize)]
struct CompleteBody {
    /// Optional explicit admin folder segment.
    admin_folder: Option<String>,
    /// Required when overwriting an existing non-default prefix.
    #[serde(default)]
    wipe_confirmed: bool,
}

#[derive(Debug, Serialize)]
struct CompleteResponse {
    admin_prefix: String,
    admin_token: String,
    env_path: String,
    next_step: String,
}

/// `GET /install/api/status`
#[get("/install/api/status")]
async fn install_status(root: web::Data<InstallRoot>) -> impl Responder {
    let available = install_artefacts_present(&root.0);
    HttpResponse::Ok().json(StatusResponse {
        available,
        wipe_required: available && existing_prefix_needs_wipe(&root.0),
        rename_after_success: format!("mv {INSTALL_DIR_NAME} {INSTALL_OFF_DIR_NAME}"),
    })
}

/// `POST /install/api/complete`
#[post("/install/api/complete")]
async fn install_complete(
    root: web::Data<InstallRoot>,
    body: web::Json<CompleteBody>,
) -> Result<HttpResponse, actix_web::Error> {
    if !install_artefacts_present(&root.0) {
        return Ok(HttpResponse::NotFound().finish());
    }
    match run_install_write(&InstallWriteOptions {
        admin_folder: body.admin_folder.clone(),
        wipe_confirmed: body.wipe_confirmed,
    }) {
        Ok(result) => Ok(HttpResponse::Ok().json(CompleteResponse {
            admin_prefix: result.admin_prefix,
            admin_token: result.admin_token,
            env_path: result.env_path.display().to_string(),
            next_step: format!(
                "Run `mv {INSTALL_DIR_NAME} {INSTALL_OFF_DIR_NAME}` so /install stops being served."
            ),
        })),
        Err(InstallEnvError::WipeRequired) => {
            Ok(HttpResponse::Conflict().json(serde_json::json!({
                "error": "wipe_required",
                "message": "I understand this will wipe my shop files and database."
            })))
        }
        Err(InstallEnvError::InvalidPrefix(message)) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "invalid_admin_folder",
                "message": message
            })))
        }
        Err(InstallEnvError::Io(error)) => Err(actix_web::error::ErrorInternalServerError(error)),
    }
}

/// Registers install API + static files when artefacts exist.
pub fn configure_install(cfg: &mut web::ServiceConfig, root: &std::path::Path) {
    if !install_artefacts_present(root) {
        return;
    }
    let dist = install_dir(root).join("dist");
    cfg.app_data(web::Data::new(InstallRoot(root.to_path_buf())))
        .service(install_status)
        .service(install_complete)
        .service(Files::new("/install", dist).index_file("index.html"));
}

/// Convenience for [`shop_root`] at configure time.
pub fn configure_install_from_env(cfg: &mut web::ServiceConfig) {
    configure_install(cfg, &shop_root());
}
