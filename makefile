###############################
build.wasm:
	wasm-pack build

###############################
init.web: build.wasm
	yarn --cwd web;

build.web: init.web
	yarn --cwd web build

start: init.web
	yarn --cwd web start

###############################
.DEFAULT: build.web

# We don't need this file anymore