

build:
	cargo build --target wasm32-wasi --release

test: build
	@echo ""
	@wazero\
		run\
		-mount tests/\
		-mount test-output/\
		target/wasm32-wasi/release/imageresize.wasm\
		./tests/statue.jpg\
		./test-output/statue-10.jpg\
		-10%

bacalhau:
	./bin/darwin_arm64/bacalhau wasm run imageresize.wasm\
	    -i src=http://127.0.0.1:8081/statue.jpg,dst=/inputs/croc.png \
		-- /inputs/croc.png  /outputs/resized.jpg -10%

