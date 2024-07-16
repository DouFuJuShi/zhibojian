watch:
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

reload:
	RUST_BACKTRACE=1 && dx serve --hot-reload --platform desktop