use crate::{allmodel::user_models::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put, web::{Data, Json, Path}, HttpResponse
};
use mongodb::bson::oid::ObjectId;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    // let existing_user = db.find_user_by_email(&new_user.email).await?;
    let user_detailss = db.find_user_by_email(&new_user.email).await;
    if user_detailss.is_ok(){
        println!("User details: {:?}", user_detailss);
        return HttpResponse::Ok().json("The user is already excited!");
    }
    // match user_detailss {
    //     Ok(user) => HttpResponse::Ok().json("Email address already exists ");
    //     // Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
    // if user_detailss {
    //     return HttpResponse::BadRequest().json("Email address already exists");
    // }
    let user_detail = db.create_user(data).await;
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secretKey").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    let token_str = claims.sign_with_key(&key).unwrap();
    assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
     println!("{} {}", token_str,"hello bro");
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
    
}
#[get("/user/{id}")]
    pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
        let id = path.into_inner();
        if id.is_empty() {
            return HttpResponse::BadRequest().body("invalid ID");
        }
        let user_detail = db.get_user(&id).await;
        match user_detail {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

#[put("/updateuser/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

    //belwo code is working will but with method post 

#[post("/deleteuser/{id}")]
pub async fn delete_user(
    db: Data<MongoRepo>, path: Path<String>)->HttpResponse{
        let id = path.into_inner();
        if id.is_empty() {
            return HttpResponse::BadRequest().body("invalid ID");
        };
        let result = db.delete_user(&id).await;
        match result {
            Ok(res) => {
                if res.deleted_count == 1 {
                    return HttpResponse::Ok().json("User successfully deleted!");
                } else {
                    return HttpResponse::NotFound().json("User with specified ID not found!");
                }
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    //belwo code is working will but with method delete 
// #[delete("/user/{id}")]
//     pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//         let id = path.into_inner();
//         if id.is_empty() {
//             return HttpResponse::BadRequest().body("invalid ID");
//         };
//         let result = db.delete_user(&id).await;
//         match result {
//             Ok(res) => {
//                 if res.deleted_count == 1 {
//                     return HttpResponse::Ok().json("User successfully deleted!");
//                 } else {
//                     return HttpResponse::NotFound().json("User with specified ID not found!");
//                 }
//             }
//             Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//         }
//     }