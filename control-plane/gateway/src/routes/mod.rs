use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::middleware::auth as auth_mw;
use crate::state::AppState;

mod alerts;
mod auth;
mod certificates;
mod config;
mod containers;
mod inventory;
mod jobs;
mod kubernetes;
mod metrics;
mod networks;
mod nodes;
mod notifications;
mod ops;
mod plugins;
mod policies;
mod storage;
mod users;
mod vms;

pub fn create_routes(state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/refresh", post(auth::refresh));

    let protected = Router::new()
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        .route("/api/auth/change-password", post(auth::change_password))
        .route(
            "/api/auth/tokens",
            get(auth::list_tokens).post(auth::create_api_token),
        )
        .route(
            "/api/auth/tokens/:id",
            axum::routing::delete(auth::delete_token),
        )
        .route("/api/users", get(users::list).post(users::create))
        .route(
            "/api/users/:id",
            get(users::get_by_id)
                .put(users::update)
                .delete(users::delete),
        )
        .route("/api/nodes", get(nodes::list))
        .route("/api/nodes/register", post(nodes::register))
        .route(
            "/api/nodes/:id",
            get(nodes::get_by_id)
                .put(nodes::update)
                .delete(nodes::delete),
        )
        .route(
            "/api/nodes/:id/metrics",
            get(nodes::metrics_history).post(nodes::submit_metrics),
        )
        .route("/api/nodes/:id/metrics/latest", get(nodes::metrics_latest))
        .route("/api/nodes/:id/command", post(nodes::execute_command))
        .route("/api/containers", get(containers::list))
        .route(
            "/api/containers/:id",
            get(containers::get_by_id).delete(containers::delete),
        )
        .route("/api/containers/:id/start", post(containers::start))
        .route("/api/containers/:id/stop", post(containers::stop))
        .route("/api/containers/:id/restart", post(containers::restart))
        .route("/api/containers/:id/logs", get(containers::logs))
        .route("/api/vms", get(vms::list).post(vms::create))
        .route(
            "/api/vms/:id",
            get(vms::get_by_id).put(vms::update).delete(vms::delete),
        )
        .route("/api/vms/:id/start", post(vms::start))
        .route("/api/vms/:id/stop", post(vms::stop))
        .route("/api/vms/:id/restart", post(vms::restart))
        .route("/api/vms/:id/snapshot", post(vms::create_snapshot))
        .route("/api/vms/:id/snapshots", get(vms::list_snapshots))
        .route("/api/vms/:id/backup", post(vms::backup))
        .route("/api/k8s/nodes", get(kubernetes::nodes))
        .route("/api/k8s/pods", get(kubernetes::pods))
        .route("/api/k8s/deployments", get(kubernetes::deployments))
        .route("/api/k8s/services", get(kubernetes::services))
        .route("/api/k8s/namespaces", get(kubernetes::namespaces))
        .route("/api/monitoring/overview", get(metrics::overview))
        .route(
            "/api/monitoring/prometheus/query",
            get(metrics::prometheus_query),
        )
        .route("/api/monitoring/alerts", get(alerts::list))
        .route("/api/monitoring/alerts/:id/ack", post(alerts::acknowledge))
        .route(
            "/api/notifications/preferences",
            get(notifications::get_prefs).put(notifications::update_prefs),
        )
        .route("/api/jobs", get(jobs::list).post(jobs::create))
        .route("/api/jobs/:id", get(jobs::get_by_id))
        .route("/api/networks", get(networks::list).post(networks::create))
        .route(
            "/api/networks/:id",
            get(networks::get_by_id).delete(networks::delete),
        )
        .route(
            "/api/storage/pools",
            get(storage::list_pools).post(storage::create_pool),
        )
        .route(
            "/api/storage/pools/:id",
            get(storage::get_pool)
                .put(storage::update_pool)
                .delete(storage::delete_pool),
        )
        .route(
            "/api/storage/volumes",
            get(storage::list_volumes).post(storage::create_volume),
        )
        .route(
            "/api/storage/volumes/:id",
            axum::routing::put(storage::update_volume).delete(storage::delete_volume),
        )
        .route(
            "/api/inventory",
            get(inventory::list).post(inventory::create),
        )
        .route(
            "/api/inventory/:id",
            get(inventory::get_by_id)
                .put(inventory::update)
                .delete(inventory::delete),
        )
        .route("/api/policies", get(policies::list).post(policies::create))
        .route(
            "/api/policies/:id",
            get(policies::get_by_id)
                .put(policies::update)
                .delete(policies::delete),
        )
        .route(
            "/api/certificates",
            get(certificates::list).post(certificates::create),
        )
        .route("/api/certificates/:id/revoke", post(certificates::revoke))
        .route("/api/config", get(config::list).post(config::set))
        .route("/api/config/:key", get(config::get_by_key))
        .route("/api/audit", get(ops::list_audit))
        .route(
            "/api/backups",
            get(ops::list_backups).post(ops::create_backup),
        )
        .route(
            "/api/backups/:id",
            axum::routing::delete(ops::delete_backup),
        )
        .route("/api/backups/:id/restore", post(ops::restore_backup))
        .route("/api/plugins", get(plugins::list).post(plugins::create))
        .route(
            "/api/plugins/:id",
            get(plugins::get_by_id)
                .put(plugins::update)
                .delete(plugins::delete),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_mw::auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
