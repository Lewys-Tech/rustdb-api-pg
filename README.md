rustdb-api-pg 🐘

An async key-value HTTP API built with Tokio, Axum and SQLx, backed by PostgreSQL, with an integration test suite that runs against real databases.

This is the third version of the same API, each built to learn what the next layer does for you:

Demo

curl -i -X POST http://127.0.0.1:8080/set -d "city=nairobi"   # 200 OK


curl http://127.0.0.1:8080/get/city                           # nairobi


curl -X POST http://127.0.0.1:8080/set -d "city=mombasa"      # overwrites


curl http://127.0.0.1:8080/get/city                           # mombasa


curl -i -X DELETE http://127.0.0.1:8080/delete/city           # 204 No Content


curl -i http://127.0.0.1:8080/get/city                        # 404 Not Found



Data lives in Postgres, so it survives server restarts with no replay step.

Getting Started


Prerequisites
Rust (developed on 1.93)


PostgreSQL 14 or later

1. Set up Postgres (Ubuntu)



sudo apt install postgresql postgresql-contrib


sudo -u postgres psql

CREATE USER lewis WITH PASSWORD 'your-password';
CREATE DATABASE rustdb OWNER lewis;
\q

2. Configure the connection

export DATABASE_URL=postgres://USER:PASSWORD@localhost/rustdb

3. Run

cargo run

The table is created on startup if it doesn't exist, and the server listens on 127.0.0.1:8080


Resources

Tokio tutorial


Axum documentation


SQLx documentation


PostgreSQL documentation


The Rust Book: Testing
