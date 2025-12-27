1 - agora com o middleware criado, vamos envolver as rotas autenticadas no main.rs


 HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(
                web::scope("/api")
                .wrap(from_fn(middleware::auth::verify_jwt))
                .service(controllers::me::get_profile)
                .service(controllers::me::update_profile)
            )
    })
    .bind(("0.0.0.0", 8080))?