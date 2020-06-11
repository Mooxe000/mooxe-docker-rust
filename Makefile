build:
	buildah bud --layers -t mooxe/rust .

rebuild:
	buildah bud --no-cache -t mooxe/rust .

in:
	podman run --rm -ti \
		--name=rust \
		-v $$(pwd):/root/rust \
			mooxe/rust /bin/bash

push:
	podman push mooxe/rust
