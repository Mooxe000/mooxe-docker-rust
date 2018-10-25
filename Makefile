build:
	docker build -t mooxe/rust .

rebuild:
	docker build --no-cache -t mooxe/rust .

in:
	docker run --rm -ti \
		--name=rust \
		-v $$(pwd):/root/rust \
			mooxe/rust /bin/bash

push:
	docker push mooxe/rust
