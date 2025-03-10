# Rust_Postgres_axum_sqlx_dioxus

this backend use axum for restfull api, sqlx to connect to postgres database in podman container

- podman run -d -it --name <>  -e POSTGRES_PASSWORD=<> -p 5432:5432 docker.io/library/postgres:15 
- cargo check
- sqlx database create
- sqlx migrate run
- cargo run


