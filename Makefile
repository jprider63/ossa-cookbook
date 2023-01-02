

build:
	npx tailwindcss -i ./style/style.css -o ./dist/style.css && cargo build

watch:
	npx tailwindcss -i ./style/style.css -o ./dist/style.css --watch
