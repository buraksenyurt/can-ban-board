cd frontend && rm -rf pkg && rm -rf public/pkg && wasm-pack build --target web && mv ./pkg ./public/pkg
cd ..
docker compose build --no-cache
docker compose up