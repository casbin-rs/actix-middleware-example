// Copyright 2022 The casbin Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    config::db::Pool,
    constants,
    models::{
        post::{DeletePost, NewPost},
        response::ResponseBody,
    },
    services::post_service,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use chrono::{NaiveDateTime, Utc};

//#[get("/posts")]
pub async fn find_all_public(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::find_all_public(&pool) {
        Ok(posts) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, posts))),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/post/{id}")]
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::find_by_id_public(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(post) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, post))),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/admin/posts")]
pub async fn find_all(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::find_all(req, &pool) {
        Ok(posts) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, posts))),
        Err(err) => Ok(err.response()),
    }
}

//#[post("/post")]
pub async fn insert(new_post: web::Json<NewPost>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut post = new_post.into_inner();
    post.created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    match post_service::insert(post, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

//#[get("/admin/post/{id}")]
pub async fn find_by_id_admin(
    req: HttpRequest,
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::find_by_id(req, id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(post) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, post))),
        Err(err) => Ok(err.response()),
    }
}

//#[delete("/admin/post/{id}")]
pub async fn delete(
    id: web::Path<String>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let delete_post = DeletePost {
        is_deleted: true,
        deleted_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
    };
    match post_service::delete(
        id.into_inner().parse::<i32>().unwrap(),
        req,
        delete_post,
        &pool,
    ) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
