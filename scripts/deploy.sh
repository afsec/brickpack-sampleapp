#!/bin/bash
PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")
cargo build --release --target $(rustup target list | awk '/musl.*installed/{print $1}')
mkdir -p ./dist
cp -v ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}
strip ./dist/${PACKAGE_NAME}
IMAGE_NAME=$(git remote get-url --all origin | cut -d ':' -f 2 | sed "s/\..*$//g")

deploy() {
	echo "Deploying ${IMAGE_NAME}..."
	docker build -t ${IMAGE_NAME} .
	docker run --name ${PACKAGE_NAME} -d -p 8000:8000 ${IMAGE_NAME}
	docker logs -f ${PACKAGE_NAME}
}

undeploy() {
	echo "Undeploying ${IMAGE_NAME}..."
	docker rm -f ${PACKAGE_NAME}
	docker rmi ${IMAGE_NAME}:latest
}


[ X"${1}" == X"-u" ] && undeploy || deploy
