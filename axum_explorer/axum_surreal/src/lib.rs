use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Option<String>,
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: f64,
}
// use the below for parsing the response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmployeeResp {
    pub id: Thing,
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: f64,
}
use axum::extract::State;
use serde_json::json;
use std::sync::Arc;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
// use surrealdb::sql::Value;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

pub async fn conn_sdb() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("test_ns").use_db("test_db").await.unwrap();
    db
}

// this will hold the client
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Client>>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

pub async fn create_employee(
    State(state): State<AppState>,
    Json(payload): Json<Employee>,
) -> impl IntoResponse {
    let db = &state.db;
    let employee_id = format!("employee:{}", Uuid::new_v4());
    let employee = Employee {
        id: Some(employee_id.clone()),
        ..payload
    };
    let _emp: Option<Record> = db
        .create("employee")
        .content(employee.clone())
        .await
        .unwrap();
    (StatusCode::CREATED, Json(employee))
    // "employee created.\n".into()
}

pub async fn get_employee(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = &state.db;
    let emp: Option<Employee> = db.select(("employee", id)).await.unwrap();
    match emp {
        Some(e) => (StatusCode::OK, Json(serde_json::to_string(&e).unwrap())),
        None => (StatusCode::NOT_FOUND, Json("Not found".to_string())),
    }
}
pub async fn get_employees(State(state): State<AppState>) -> impl IntoResponse {
    println!("Entering get employees...");
    let db = &state.db;
    let emp: Vec<EmployeeResp> = db.select("employee").await.unwrap();
    println!("sending total {} records", emp.len());
    // let emp_json = json!()
    (
        StatusCode::OK,
        Json(json!(serde_json::to_string(&emp).unwrap())),
    )
    // (StatusCode::OK, Json(serde_json::to_value(&emp).unwrap()))
}
