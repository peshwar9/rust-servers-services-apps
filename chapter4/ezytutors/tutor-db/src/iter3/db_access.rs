use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // Prepare SQL statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // Extract result
    course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect()
}

//Return result

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    // Prepare SQL statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1 and course_id = $2",
        tutor_id, course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // Execute query
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course_row = sqlx::query!(
        "insert into ezy_course_c4 (course_id,tutor_id, course_name)  
                                  values ($1,$2,$3) returning 
                                  tutor_id, 
                                  course_id, 
                                  course_name,  
                                  posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();
    //Retrieve result
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}
