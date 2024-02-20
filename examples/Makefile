SERVER_USER ?= default_user
SERVER_IP ?= default_ip

DOCKER_HOME_PATH := ~/modules/docker
DOCKER_DEST_PATH := /home/debian/modules/docker

WASM_HOME_PATH := ~/modules/wasm
WASM_DEST_PATH := /home/debian/modules/wasm


# Find all directories with src/main.rs
W_DIRS := $(dir $(wildcard */src/main.rs))

# Find all directory names (removing trailing slashes)
# DIR_NAMES ?= $(patsubst %/src/,%,$(W_DIRS))
DIR_NAMES := fibonacci fibonacci-recursive prime-number factorial exponential

RELEASE_IMAGES := debian:bullseye-slim ubuntu:22.04 archlinux:latest
# RELEASE_IMAGES := archlinux:latest

TARGET_PLATFORM ?= linux/amd64

.PHONY: all build_docker build_wasm

all: build_docker build_wasm


# Pattern rule for building docker images
build_docker: $(DIR_NAMES)
	@for dir in $^; do \
		echo "#!/bin/sh" > $$dir/run.sh; \
		echo "/usr/local/bin/$$dir" >> $$dir/run.sh; \
		chmod +x $$dir/run.sh; \
		IMAGE_NAME=$${RELEASE_IMAGE%%:*}; \
		echo "Building with $$RELEASE_IMAGE, base is: $$IMAGE_NAME"; \
		docker build --platform $(TARGET_PLATFORM) -t nebula-function-$$dir-$$IMAGE_NAME --build-arg PROGRAM_NAME=$$dir --build-arg IMAGE=$(RELEASE_IMAGE) -f Dockerfile . || true; \
		docker save nebula-function-$$dir-$$IMAGE_NAME | gzip > $(DOCKER_HOME_PATH)/$$dir-$$IMAGE_NAME.tar.gz; \
		rm $$dir/run.sh; \
  done

# Build docker images for each distro
build_all_docker: 
	@for img in $(RELEASE_IMAGES); do \
		echo "Building with RELEASE_IMAGE=$$img"; \
		$(MAKE) build_docker RELEASE_IMAGE=$$img; \
	done

load_all_docker_locally: 
	@for img in $(RELEASE_IMAGES); do \
		echo "Loading $$img into machine"; \
		IMAGE_NAME=$${img%%:*}; \
		for dir in $(DIR_NAMES); do \
			sudo docker rmi nebula-function-$$dir-$$IMAGE_NAME || true; \
			sudo docker load -i $(DOCKER_HOME_PATH)/$$dir-$$IMAGE_NAME.tar.gz; \
		done \
	done

# Pattern rule for building wasm modules
build_wasm: 
	cargo build --release --target wasm32-wasi

deploy: deploy_docker deploy_wasm

deploy_docker: 
	@if [ "$(SERVER_USER)" = "default_user" ] || [ "$(SERVER_IP)" = "default_ip" ]; then \
		echo "Please set SERVER_USER and SERVER_IP before deploying"; \
		exit 1; \
	fi
	$(MAKE) build_docker
	@for dir in $(DIR_NAMES); do \
		IMAGE_NAME=$${RELEASE_IMAGE%%:*}; \
		echo "Deploying Docker image for $$dir-$$IMAGE_NAME"; \
		scp $(DOCKER_HOME_PATH)/$$dir-$$IMAGE_NAME.tar.gz $(SERVER_USER)@$(SERVER_IP):$(DOCKER_DEST_PATH)/; \
		echo "Executing SSH command to load Docker image for $$dir-$$IMAGE_NAME"; \
		ssh $(SERVER_USER)@$(SERVER_IP) "sudo docker rmi nebula-function-$$dir-$$IMAGE_NAME || true; sudo docker load -i $(DOCKER_DEST_PATH)/$$dir-$$IMAGE_NAME.tar.gz;"; \
	done

deploy_all_docker: 
	@for img in $(RELEASE_IMAGES); do \
		$(MAKE) deploy_docker RELEASE_IMAGE=$$img; \
	done


# deploy_docker: build_docker 
# 	@if [ "$(SERVER_USER)" = "default_user" ] || [ "$(SERVER_IP)" = "default_ip" ]; then \
# 		echo "Please set SERVER_USER and SERVER_IP before deploying"; \
# 		exit 1; \
# 	fi
# 	@for dir in $(DIR_NAMES); do \
# 		scp $(DOCKER_HOME_PATH)/$$dir.tar.gz $(SERVER_USER)@$(SERVER_IP):$(DOCKER_DEST_PATH)/; \
# 		# SSH into server and unload previous image with same name, and reload
# 		ssh $(SERVER_USER)@$(SERVER_IP) "\
# 			docker rmi nebula-function-$$dir || true; \
# 			docker load -i $(DOCKER_DEST_PATH)/$$dir.tar.gz; \
# 		"; \
#   done

deploy_wasm: build_wasm 
	@if [ "$(SERVER_USER)" = "default_user" ] || [ "$(SERVER_IP)" = "default_ip" ]; then \
		echo "Please set SERVER_USER and SERVER_IP before deploying"; \
		exit 1; \
	fi
	@for dir in $(DIR_NAMES); do \
		scp $(WASM_HOME_PATH)/$$dir.wasm $(SERVER_USER)@$(SERVER_IP):${WASM_DEST_PATH}/; \
  done

# Cleanup if you need (optional)
.PHONY: clean
clean:
	rm $(DOCKER_HOME_PATH)/*.tar.gz
	rm $(WASM_HOME_PATH)/*.wasm
