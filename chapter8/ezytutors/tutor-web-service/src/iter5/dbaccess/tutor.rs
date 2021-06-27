use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    // Prepare SQL statement
    let tutor_rows =
        sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor_c6")
            .fetch_all(pool)
            .await?;
    // Extract result

    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name.clone(),
            tutor_pic_url: tutor_row.tutor_pic_url.clone(),
            tutor_profile: tutor_row.tutor_profile.clone(),
        })
        .collect();
    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

//Return result

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor_c6 where tutor_id = $1 order by tutor_id desc",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name,
            tutor_pic_url: tutor_row.tutor_pic_url,
            tutor_profile: tutor_row.tutor_profile,
        }
    )
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    change_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    // Retrieve current tutor record:
    let tutor_row = sqlx::query!(
    "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor_c6 where tutor_id = $1",
    tutor_id
)
.fetch_one(pool)
.await
.map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    let new_tutor_record = Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: if let Some(name) = change_tutor.tutor_name {
            name
        } else {
            tutor_row.tutor_name
        },
        tutor_pic_url: if let Some(pic) = change_tutor.tutor_pic_url {
            pic
        } else {
            tutor_row.tutor_pic_url
        },
        tutor_profile: if let Some(profile) = change_tutor.tutor_profile {
            profile
        } else {
            tutor_row.tutor_profile
        },
    };

    // Prepare SQL statement
    let tutor_updated_row = sqlx::query!(
        "UPDATE ezy_tutor_c6 SET tutor_name = $1, tutor_pic_url=$2, tutor_profile=$3 where tutor_id = $4 returning tutor_id, tutor_name, tutor_pic_url, tutor_profile", 
        new_tutor_record.tutor_name, new_tutor_record.tutor_pic_url, new_tutor_record.tutor_profile, tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name,
            tutor_pic_url: tutor_row.tutor_pic_url,
            tutor_profile: tutor_row.tutor_profile,
        }
    )
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_updated_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!("insert into ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile) values ($1,$2,$3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile", new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile)
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

// Delete tutor
pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query(&format!(
        "DELETE FROM ezy_tutor_c6 where tutor_id = {}",
        tutor_id
    ))
    .execute(pool)
    .await
    .map_err(|_err| EzyTutorError::DBError("Unable to delete tutor ".into()))?;

    Ok(format!("Deleted {} record", tutor_row))
}
