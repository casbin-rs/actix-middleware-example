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

use actix_web::web;

use crate::api::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(web::resource("/login").route(web::post().to(user::login)))
                    .service(web::resource("/signup").route(web::post().to(user::signup))),
            )
            .service(
                web::scope("/user")
                    .service(web::resource("/logout").route(web::post().to(user::logout)))
                    .service(web::resource("").route(web::delete().to(user::delete_self))),
            )
            .service(
                web::scope("/admin")
                    .service(
                        web::resource("/user/{id}")
                            .route(web::delete().to(user::delete_user))
                            .route(web::get().to(user::find_by_id)),
                    )
                    .service(
                        web::resource("/post/{id}")
                            .route(web::get().to(post::find_by_id_admin))
                            .route(web::delete().to(post::delete)),
                    )
                    .service(web::resource("/users").route(web::get().to(user::find_all)))
                    .service(web::resource("/posts").route(web::get().to(post::find_all))),
            )
            .service(
                web::scope("/post")
                    .service(web::resource("/{id}").route(web::get().to(post::find_by_id)))
                    .service(web::resource("").route(web::post().to(post::insert))),
            )
            .service(
                web::scope("/posts")
                    .service(web::resource("").route(web::get().to(post::find_all_public))),
            ),
    );
}
