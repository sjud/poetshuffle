# Run docker container with the name of test_db so we can stop it by its name
# --rm will remove the container once it is stopped
# -d will run it 'detached' so that its output does not block stdin
# -p 5433:5432 will map our docker host port 5433 (the port we give to our servers database_url)
# to the containers port of 5432 which is uses to listen for connection to postgres
docker compose -f server-dev-compose.yml up --build -d
# we construct the DB url info (this should be from an env file or something) and then see if the server is running.
# see this stackoverflow for a further explanation
# https://stackoverflow.com/questions/14549270/check-if-database-exists-in-postgresql-using-shell
while [[ "$( PGPASSWORD=PASSWORD psql --port=5433 --user=postgres --host=0.0.0.0 \
-XtAc "SELECT 1 FROM pg_database WHERE datname='postgres'" )" != 1 ]]
do
  #if the server isn't running we sleep for 1
sleep 1;
done;
# we migrate the database once the docker db is up and running.
sea migrate up
# we test our server knowing that we can populate the fresh db with whatever
cargo test --package server --features dev
# we stop our container which deletes it because we passed in --rm earlier.
docker compose -f server-dev-compose.yml drop
