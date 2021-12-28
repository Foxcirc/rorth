
name = ""
args = ""

build:
	cargo build --release

here: build
	move ".\target\release\rorth.exe" ".\rh.exe"

run:
	cargo run --release -- $(args)

show:
	cargo test --release $(name) -- --nocapture

test:
	cargo test --release $(name)

clean:
	del .\rh.exe
	cargo clean > NUL 2> NUL
