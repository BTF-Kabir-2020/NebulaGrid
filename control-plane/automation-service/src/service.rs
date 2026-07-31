use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::*;

#[derive(Clone)]
pub struct JobService {
    store: Arc<Mutex<Vec<Job>>>,
}

impl JobService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn create_job(&self, req: CreateJobRequest) -> Job {
        let now = Utc::now();
        let job = Job {
            id: Uuid::new_v4(),
            name: req.name,
            job_type: req.job_type,
            status: "pending".into(),
            target_nodes: req.target_nodes,
            params: req.params,
            result: None,
            error: None,
            created_at: now,
            started_at: None,
            completed_at: None,
        };

        let job_id = job.id;
        let store = self.store.clone();

        {
            let mut jobs = store.lock().await;
            jobs.push(job.clone());
        }

        tokio::spawn(async move {
            let mut jobs = store.lock().await;
            if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                j.status = "running".into();
                j.started_at = Some(Utc::now());
            }
            drop(jobs);

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let mut jobs = store.lock().await;
            if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                j.status = "completed".into();
                j.result = Some(serde_json::json!({
                    "message": "Job completed successfully",
                    "output": "Sample playbook output"
                }));
                j.completed_at = Some(Utc::now());
            }
        });

        job
    }

    pub async fn list_jobs(&self, query: JobQuery) -> PaginatedResponse<Job> {
        let jobs = self.store.lock().await;
        let filtered: Vec<Job> = if let Some(ref status) = query.status {
            jobs.iter()
                .filter(|j| j.status == *status)
                .cloned()
                .collect()
        } else {
            jobs.clone()
        };

        let total = filtered.len() as i64;
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).max(1).min(100);
        let total_pages = (total + per_page - 1) / per_page;

        let start = ((page - 1) * per_page) as usize;
        let data: Vec<Job> = filtered
            .into_iter()
            .skip(start)
            .take(per_page as usize)
            .collect();

        PaginatedResponse {
            data,
            page,
            per_page,
            total,
            total_pages,
        }
    }

    pub async fn get_job(&self, id: Uuid) -> Option<Job> {
        let jobs = self.store.lock().await;
        jobs.iter().find(|j| j.id == id).cloned()
    }

    pub async fn get_job_logs(&self, id: Uuid) -> Result<Vec<String>, String> {
        let jobs = self.store.lock().await;
        if jobs.iter().any(|j| j.id == id) {
            Ok(vec![
                "[INFO] Job accepted".into(),
                "[INFO] Connecting to target nodes...".into(),
                "[INFO] Playbook execution started".into(),
                "[INFO] Playbook execution completed".into(),
            ])
        } else {
            Err(format!("Job {} not found", id))
        }
    }
}
