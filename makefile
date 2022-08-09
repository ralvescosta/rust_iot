release:
	cargo build --release

build-images: release
	@docker build -f ./.k8s/dockerfiles/Dockerfile.amqp . -t ralvescosta/amqp:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.dummy . -t ralvescosta/dummy:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.dump . -t ralvescosta/dump:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.grpc . -t ralvescosta/grpc:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.api . -t ralvescosta/api:latest
	@docker build -f ./.k8s/dockerfiles/Dockerfile.mqtt . -t ralvescosta/mqtt:latest