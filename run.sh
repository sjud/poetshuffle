# See test.sh for comments

docker run --name test_db --rm -d -p 5433:5432 test_db
while [[ "$( PGPASSWORD=PASSWORD psql --port=5433 --user=postgres --host=0.0.0.0 \
-XtAc "SELECT 1 FROM pg_database WHERE datname='postgres'" )" != 1 ]]
do
sleep 1;
done;
sea migrate up
# Cargo run instead of test
cargo run --bin server --features dev
# we need to stop it ourself, so data persists between re compiles
# this is what we want because data here is a result of a manual entry

# docker stop test_db