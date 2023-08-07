# CRM Axum API

* uses Axum as a web application framework (API Server)

# Endpoint Definitions & Usage 

Please see the [README](src/routes/README.md) within the `src/routes` directory.


# Rust / Axum Setup

To see documentation for the Rust crates used in this project: 
```
> cargo doc --open
```

To start the Axum server such that it will refresh with each saved code change:
```
cargo watch -x run
```

The API server runs on port 3000: 
```
http://localhost:3000/
```

### Environment Variables 

environment variables for the docker database container are stored in `crm_api/database/.env`
```
POSTGRES_PASSWORD=<password>
```

environment variables needed by the application are stored in `crm_api/.env`
```
DATABASE_URL="postgresql://<username>:<password>@localhost:<port>/<database_name>"
```

# Docker / Postgres Setup

within the `database/` directory there is an `init.sql` file which contains the SQL to create  
the `platform` db and schema. It also contains some insert statements to insert sample data.

Make sure you set the required environment variables as explained in the previous section before running these commands.

### Starting up the database container that interfaces with the Axum server

```
cd backend/crm_api/database
docker compose up
```

### Getting into the database container directly via psql

get container name
```
docker ps
```
exec into the container 
```
docker exec -it <container_name> bash
```
get into the db via `psql`
```
psql -U postgres
```
commands within psql
```
\c platform;
select * from platform.topics;
```
shutting down the docker container 
```
docker stop <container_name>
```


If you want to make updates to `init.sql` file after having started the docker container once,
you'll need to run this command within `backend/crm_api` to remove the cached version of the 
postgres DB. 
```
docker-compose down --volumes
```

