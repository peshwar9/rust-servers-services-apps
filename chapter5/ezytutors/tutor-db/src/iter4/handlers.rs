use super::db_access::*;
use super::errors::EzyTutorError;
use super::models::Course;
use super::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    web::Path((tutor_id, course_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/* curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
 -d '{"tutor_id":1, "course_name":"Course 1"}'
*/
pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::{NaiveDate};
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    ///
    #[ignore]
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = Course {
            course_id: 3,
            tutor_id: 1,
            course_name: "Third course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
