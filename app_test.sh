docker compose -f app-test-compose.yml up --build -d
export ADMIN_USER=admin@admin.admin
export ADMIN_PASS=1234
export BASE_URL=http://127.0.0.1:3001/
wasm-pack test --chrome --headless app
# we stop our container which deletes it because we passed in --rm earlier.
docker compose -f app-test-compose.yml down
