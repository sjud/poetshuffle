# The idea here is to run the server from it's local binary
# to allow easy iterations on the code
# while running the distributed components (s3,email,db) in docker
# containers
# we make local changes to our app which trunk will hot reload
# into a volume being watched by our s3 container
# that way we can combine quick iterations with actual api development against
# suitable network targets, i,e a working s3 instance, email, db etc.
docker run --name test_db --rm -d -p 5433:5432 poetshuffle_v2_test_db
while [[ "$( PGPASSWORD=PASSWORD psql --port=5433 --user=postgres --host=0.0.0.0 \
-XtAc "SELECT 1 FROM pg_database WHERE datname='postgres'" )" != 1 ]]
do
sleep 1;
done;
sea migrate up
cargo run --package server --features dev
# docker compose -f server-dev-compose.yml down
# we don't drop compose here, so it will still be running in the
# background, you can re-run run-dev.sh to recompile + ~2 seconds
# or you can just cargo run --package server --features dev