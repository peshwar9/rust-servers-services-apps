use crate::errors::EzyTutorError;
use crate::models::course::*;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare SQL statement

    let course_rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1 order by course_id desc",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(course_rows)
}

//Return result

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    // Prepare SQL statement
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, EzyTutorError> {
    let course_row= sqlx::query_as!(Course,"insert into ezy_course_c6 (tutor_id, course_name, course_description,course_duration, course_level, course_format, course_language, course_structure, course_price) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning tutor_id, course_id,course_name, course_description, course_duration, course_level, course_format, course_language, course_structure, course_price, posted_time", 
    new_course.tutor_id, new_course.course_name, new_course.course_description,
    new_course.course_duration, new_course.course_level, new_course.course_format, new_course.course_language, new_course.course_structure, new_course.course_price)
    .fetch_one(pool)
    .await?;

    Ok(course_row)
}

// Delete course
pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    // Prepare SQL statement
    let course_row = sqlx::query!(
        "DELETE FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .execute(pool)
    .await?;
    Ok(format!("Deleted {} record", course_row))
}

// Update course details
pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    // Retrieve current record

    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Course id not found".into()))?;

    // Construct the parameters for update:

    let name: String = if let Some(name) = update_course.course_name {
        name
    } else {
        current_course_row.course_name
    };
    let description: String = if let Some(desc) = update_course.course_description {
        desc
    } else {
        current_course_row.course_description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.course_format {
        format
    } else {
        current_course_row.course_format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.course_structure {
        structure
    } else {
        current_course_row.course_structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.course_duration {
        duration
    } else {
        current_course_row.course_duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.course_level {
        level
    } else {
        current_course_row.course_level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.course_language {
        language
    } else {
        current_course_row.course_language.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.course_price {
        price
    } else {
        current_course_row.course_price.unwrap_or_default()
    };

    // Prepare SQL statement
    let course_row =
        sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c6 set course_name = $1, course_description = $2, course_format = $3, 
        course_structure = $4, course_duration = $5, course_price = $6, course_language = $7, 
        course_level = $8 where tutor_id = $9 and course_id = $10 returning tutor_id, course_id, 
        course_name, course_description, course_duration, course_level, course_format, 
        course_language, course_structure, course_price, posted_time ", name, description, format, 
        structure, duration, price, language,level, tutor_id, course_id
    )
        .fetch_one(pool)
        .await;
    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
