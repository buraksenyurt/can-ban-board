use crate::api::*;
use crate::db::ExecuteResult;
use crate::handlers::app_error::AppError;
use crate::models::WorkItem;
use crate::state::AppState;
use crate::utility::calculate_planned_finish_time;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use chrono::Local;
use log::{error, info};
use shared::*;
use validator::Validate;

pub struct WorkItemHandler {}

impl WorkItemHandler {
    pub async fn create(
        item: web::Json<CreateWorkItemRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let payload = item.into_inner();

        if let Err(validation_errors) = payload.validate() {
            return HttpResponse::BadRequest().json(validation_errors);
        }

        if let Ok(count) = db.get_count() {
            if count >= 5 {
                return HttpResponse::NotAcceptable()
                    .json("There can be only 5 work items at the same time");
            }
        }

        let new_item = WorkItem {
            id: 0,
            title: payload.title,
            duration: payload.duration,
            duration_type: payload.duration_type,
            size: payload.size,
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
            finish_date: None,
        };

        match db.add_work_item(&new_item) {
            Ok(ExecuteResult::WorkItemCreated(id)) => {
                info!("{id}, New item has been added");
                let planned_time = calculate_planned_finish_time(&new_item);
                let response = WorkItemResponse {
                    id,
                    title: new_item.title,
                    duration: new_item.duration,
                    duration_type: new_item.duration_type,
                    size: new_item.size,
                    status: new_item.status,
                    finish_date: planned_time,
                };
                HttpResponse::Created().json(response)
            }
            Ok(_) => HttpResponse::BadRequest().body("Unexpected error during item creation"),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn update_state(
        body: web::Json<UpdateStatusRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let payload = body.into_inner();
        info!("{:?}", payload);

        match db.update_work_item_status(&payload) {
            Ok(ExecuteResult::UpdatedOneRow) => HttpResponse::Ok().finish(),
            Ok(ExecuteResult::NoRowsAffected) => HttpResponse::NotFound().json("Item not found"),
            Ok(_) => HttpResponse::BadRequest().body("Unexpected result during status update"),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn move_to_archive(
        body: web::Json<MoveToArchiveRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let item_id = body.into_inner().id;

        match db.move_to_archive(item_id) {
            Ok(ExecuteResult::MovedToArchive) => HttpResponse::Ok().finish(),
            Ok(ExecuteResult::NoRowsAffected) => HttpResponse::NotFound().json("Item not found"),
            Ok(_) => HttpResponse::BadRequest().body("Unexpected result during archiving"),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn delete(id: web::Path<u32>, data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let item_id = *id;

        match db.delete(item_id) {
            Ok(ExecuteResult::Deleted) => HttpResponse::Ok().finish(),
            Ok(ExecuteResult::NoRowsAffected) => HttpResponse::NotFound().json("Item not found"),
            Ok(_) => HttpResponse::BadRequest().body("Unexpected result during deletion"),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get(id: web::Path<u32>, data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let item_id = *id;

        match db.get_item(item_id) {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(AppError::NotFound) => HttpResponse::NotFound().json("Item not found"),
            Err(AppError::DatabaseError(e)) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get_actives(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();

        match db.get_all(true) {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get_all(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();

        match db.get_all(false) {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get_count(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();

        match db.get_count() {
            Ok(result) => {
                info!("Total items {}", result);
                HttpResponse::Ok().json(result)
            }
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}