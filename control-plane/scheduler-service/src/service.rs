use std::sync::Mutex;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub jobs: Vec<Job>,
    pub schedules: Vec<JobSchedule>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            schedules: Vec::new(),
        }
    }
}

pub struct SchedulerService {
    state: Mutex<AppState>,
}

impl SchedulerService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn create_job(&self, req: CreateJobRequest) -> Job {
        let now = Utc::now();
        let scheduled_at = req
            .scheduled_at
            .as_ref()
            .and_then(|s| s.parse().ok());
        let job = Job {
            id: Uuid::new_v4(),
            name: req.name,
            job_type: req.job_type,
            target_id: req.target_id,
            payload: req.payload,
            status: if scheduled_at.is_some() {
                "scheduled".into()
            } else {
                "pending".into()
            },
            scheduled_at,
            started_at: None,
            completed_at: None,
            error: None,
            created_at: now,
            updated_at: now,
        };
        let mut state = self.state.lock().unwrap();
        state.jobs.insert(0, job.clone());
        job
    }

    pub fn list_jobs(&self, page: i64, per_page: i64) -> PaginatedResponse<Job> {
        let state = self.state.lock().unwrap();
        paginate(&state.jobs, page, per_page)
    }

    pub fn get_job(&self, id: Uuid) -> Option<Job> {
        let state = self.state.lock().unwrap();
        state.jobs.iter().find(|j| j.id == id).cloned()
    }

    pub fn create_schedule(&self, req: CreateScheduleRequest) -> JobSchedule {
        let now = Utc::now();
        let schedule = JobSchedule {
            id: Uuid::new_v4(),
            name: req.name,
            job_type: req.job_type,
            cron_expr: req.cron_expr,
            target_id: req.target_id,
            payload: req.payload,
            enabled: true,
            last_run_at: None,
            next_run_at: Some(now + Duration::hours(1)),
            created_at: now,
            updated_at: now,
        };
        let mut state = self.state.lock().unwrap();
        state.schedules.push(schedule.clone());
        schedule
    }

    pub fn list_schedules(&self, page: i64, per_page: i64) -> PaginatedResponse<JobSchedule> {
        let state = self.state.lock().unwrap();
        paginate(&state.schedules, page, per_page)
    }

    pub fn set_schedule_enabled(&self, id: Uuid, enabled: bool) -> Option<JobSchedule> {
        let mut state = self.state.lock().unwrap();
        let schedule = state.schedules.iter_mut().find(|s| s.id == id)?;
        schedule.enabled = enabled;
        schedule.updated_at = Utc::now();
        Some(schedule.clone())
    }

    pub fn run_due_schedules(&self) -> Vec<Job> {
        let now = Utc::now();
        let mut created = Vec::new();
        let mut state = self.state.lock().unwrap();
        for schedule in state.schedules.iter_mut() {
            if !schedule.enabled {
                continue;
            }
            let due = schedule
                .next_run_at
                .map(|t| t <= now)
                .unwrap_or(false);
            if !due {
                continue;
            }
            let job = Job {
                id: Uuid::new_v4(),
                name: schedule.name.clone(),
                job_type: schedule.job_type.clone(),
                target_id: schedule.target_id.clone(),
                payload: schedule.payload.clone(),
                status: "pending".into(),
                scheduled_at: Some(now),
                started_at: None,
                completed_at: None,
                error: None,
                created_at: now,
                updated_at: now,
            };
            schedule.last_run_at = Some(now);
            schedule.next_run_at = Some(now + Duration::hours(1));
            schedule.updated_at = now;
            created.push(job);
        }
        for job in created.iter().rev() {
            state.jobs.insert(0, job.clone());
        }
        created
    }
}

fn paginate<T: Clone + serde::Serialize>(
    items: &[T],
    page: i64,
    per_page: i64,
) -> PaginatedResponse<T> {
    let page = page.max(1);
    let per_page = per_page.clamp(1, 100);
    let total = items.len() as i64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;
    let offset = ((page - 1) * per_page) as usize;
    let data = items.iter().skip(offset).take(per_page as usize).cloned().collect();
    PaginatedResponse {
        data,
        page,
        per_page,
        total,
        total_pages,
        has_next: page < total_pages,
        has_prev: page > 1,
    }
}
