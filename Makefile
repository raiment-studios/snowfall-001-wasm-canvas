# --------------------------------------------------------------------------- #
# CONFIGURATION
# --------------------------------------------------------------------------- #

PROJ=snowfall-001-wasm-canvas

# --------------------------------------------------------------------------- #
# ensure
# --------------------------------------------------------------------------- #

.PHONY: ensure
ensure:
	rm -rf __temp .git
	rustup target add wasm32-unknown-unknown
	which wasm-bindgen || cargo install wasm-bindgen-cli && \
		cargo update -p wasm-bindgen --precise 0.2.95
	which mprocs || cargo install mprocs
	npm install

# --------------------------------------------------------------------------- #
# build
# --------------------------------------------------------------------------- #

.PHONY: build
build:
	rm -rf dist && mkdir -p dist
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen \
		--out-dir target \
		--target web target/wasm32-unknown-unknown/release/$(PROJ).wasm
	cp src/index.html dist/
	cp target/$(PROJ).js dist/
	cp target/$(PROJ)_bg.wasm dist/
	echo "$(shell DATE)" > dist/build-timestamp.txt

# --------------------------------------------------------------------------- #
# dev
# --------------------------------------------------------------------------- #

.PHONY: dev dev-watch
dev-watch:
	npx nodemon \
		--watch src --watch assets \
		--ext rs,html,css,js,png,jpg,otf,blend \
		--exec "make build || exit 1" \

dev: init ensure
	-zellij action rename-pane "$(PWD)"
	mprocs \
		--names "dev-watch,run-server" \
		"make dev-watch" \
		"make run-server" 
	-zellij action rename-pane ""

# --------------------------------------------------------------------------- #
# run
# --------------------------------------------------------------------------- #

.PHONY: run run-server
run: build	
	$(MAKE) run-server

run-server:
	npx serve --cors --listen 8099 dist


# --------------------------------------------------------------------------- #
# publish
# --------------------------------------------------------------------------- #

.PHONY: publish publish-source publish-deploy

publish-source:
	-gh repo create raiment-studios/$(PROJ) --public
	rm -rf __temp .git
	git clone git@github.com:raiment-studios/$(PROJ).git __temp
	mv __temp/.git .
	rm -rf __temp
	git config user.email ridley.grenwood.winters@gmail.com
	git config user.name "Ridley Winters"
	git add .
	git commit -m "Automated commit from monorepo"
	git push
	rm -rf .git

publish: build publish-source publish-deploy

publish-deploy:
	@echo "Publishing..."
	deno install -Arf --global jsr:@deno/deployctl
	asdf reshim deno
	cd dist && deployctl \
		deploy --project=$(PROJ) --prod \
		https://jsr.io/@std/http/1.0.7/file_server.ts

# --------------------------------------------------------------------------- #
# clean
# --------------------------------------------------------------------------- #

.PHONY: clean
clean:
	git clean -Xdf