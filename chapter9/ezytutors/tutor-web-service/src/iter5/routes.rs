use crate::handlers::{course::*, general::*, tutor::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_details))
            .route(
                "/{tutor_id}/{course_id}",
                web::put().to(update_course_details),
            )
            .route("/{tutor_id}/{course_id}", web::delete().to(delete_course)),
    );
}

pub fn tutor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tutors")
            .route("/", web::post().to(post_new_tutor))
            .route("/", web::get().to(get_all_tutors))
            .route("/{tutor_id}", web::get().to(get_tutor_details))
            .route("/{tutor_id}", web::put().to(update_tutor_details))
            .route("/{tutor_id}", web::delete().to(delete_tutor)),
    );
}
