release:
	cargo build --release --locked

minikube:
	@minikube start
	@eval $(minikube -p minikube docker-env)

build-images:
	@docker build -f ./.k8s/dockerfiles/Dockerfile.amqp . -t ralvescosta/amqp:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.dummy . -t ralvescosta/dummy:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.dump . -t ralvescosta/dump:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.grpc . -t ralvescosta/grpc:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.api . -t ralvescosta/api:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.mqtt . -t ralvescosta/mqtt:latest


# maybe some day we use this
musl-install:
	@rustup target add x86_64-unknown-linux-musl
	@sudo apt update && apt sudo install -y musl-tools musl-dev

release-musl:
	cargo build --target x86_64-unknown-linux-musl --release --locked