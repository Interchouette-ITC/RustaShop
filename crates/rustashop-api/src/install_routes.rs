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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::install_fs::install_dist_index;
    use actix_web::{test, App};
    use std::fs;

    #[actix_web::test]
    #[allow(clippy::await_holding_lock)]
    async fn status_ok_when_dist_present() {
        let _guard = crate::install_env::INSTALL_PROCESS_ENV_LOCK
            .lock()
            .expect("lock");
        unsafe {
            std::env::remove_var(crate::install_env::ENV_FILE_ENV);
        }
        let dir = tempfile_dir("status-ok");
        let index = install_dist_index(&dir);
        fs::create_dir_all(index.parent().expect("parent")).expect("mkdir");
        fs::write(&index, "<!doctype html>").expect("write");

        let app =
            test::init_service(App::new().configure(|cfg| configure_install(cfg, &dir))).await;
        let req = test::TestRequest::get()
            .uri("/install/api/status")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["available"], true);
        assert_eq!(body["wipe_required"], false);
    }

    #[actix_web::test]
    async fn routes_absent_without_dist() {
        let dir = tempfile_dir("status-absent");
        let app =
            test::init_service(App::new().configure(|cfg| configure_install(cfg, &dir))).await;
        let req = test::TestRequest::get()
            .uri("/install/api/status")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn complete_returns_not_found_without_dist() {
        let dir = tempfile_dir("complete-absent");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(InstallRoot(dir.clone())))
                .service(install_complete),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/install/api/complete")
            .set_json(serde_json::json!({ "wipe_confirmed": true }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
        let _ = fs::remove_dir_all(&dir);
    }

    #[actix_web::test]
    #[allow(clippy::await_holding_lock)]
    async fn complete_success_wipe_and_invalid_prefix() {
        let _guard = crate::install_env::INSTALL_PROCESS_ENV_LOCK
            .lock()
            .expect("lock");
        let dir = tempfile_dir("complete-ok");
        let index = install_dist_index(&dir);
        fs::create_dir_all(index.parent().expect("parent")).expect("mkdir");
        fs::write(&index, "<!doctype html>").expect("write");
        let env_path = dir.join(".env");
        fs::write(&env_path, "RUSTASHOP_ADMIN_API_PREFIX=alreadypfx1\n").expect("env");
        unsafe {
            std::env::set_var(crate::install_fs::ROOT_ENV, &dir);
            std::env::set_var(crate::install_env::ENV_FILE_ENV, &env_path);
        }

        let app =
            test::init_service(App::new().configure(|cfg| configure_install(cfg, &dir))).await;

        let conflict = test::TestRequest::post()
            .uri("/install/api/complete")
            .set_json(serde_json::json!({
                "admin_folder": "newfolderok1",
                "wipe_confirmed": false
            }))
            .to_request();
        let conflict_resp = test::call_service(&app, conflict).await;
        assert_eq!(conflict_resp.status(), 409);
        let conflict_body: serde_json::Value = test::read_body_json(conflict_resp).await;
        assert_eq!(conflict_body["error"], "wipe_required");

        let bad = test::TestRequest::post()
            .uri("/install/api/complete")
            .set_json(serde_json::json!({
                "admin_folder": "carts",
                "wipe_confirmed": true
            }))
            .to_request();
        let bad_resp = test::call_service(&app, bad).await;
        assert_eq!(bad_resp.status(), 400);
        let bad_body: serde_json::Value = test::read_body_json(bad_resp).await;
        assert_eq!(bad_body["error"], "invalid_admin_folder");

        let ok = test::TestRequest::post()
            .uri("/install/api/complete")
            .set_json(serde_json::json!({
                "admin_folder": "newfolderok1",
                "wipe_confirmed": true
            }))
            .to_request();
        let ok_resp = test::call_service(&app, ok).await;
        assert!(ok_resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(ok_resp).await;
        assert_eq!(body["admin_prefix"], "newfolderok1");
        assert!(body["admin_token"].as_str().unwrap().len() >= 16);
        assert!(body["next_step"]
            .as_str()
            .unwrap()
            .contains(INSTALL_OFF_DIR_NAME));

        let status = test::TestRequest::get()
            .uri("/install/api/status")
            .to_request();
        let status_resp = test::call_service(&app, status).await;
        let status_body: serde_json::Value = test::read_body_json(status_resp).await;
        assert_eq!(status_body["wipe_required"], true);

        unsafe {
            std::env::remove_var(crate::install_fs::ROOT_ENV);
            std::env::remove_var(crate::install_env::ENV_FILE_ENV);
        }
        let _ = fs::remove_dir_all(&dir);
    }

    fn tempfile_dir(label: &str) -> std::path::PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "rustashop-install-routes-{label}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("tmpdir");
        dir
    }
}
