SERVER_USER ?= default_user
SERVER_IP ?= default_ip

DOCKER_HOME_PATH := ~/modules/docker
DOCKER_DEST_PATH := /home/debian/modules/docker

WASM_HOME_PATH := ~/modules/wasm
WASM_DEST_PATH := /home/debian/modules/wasm


# Find all directories with src/main.rs
W_DIRS := $(dir $(wildcard */src/main.rs))

# Find all directory names (removing trailing slashes)
DIR_NAMES := $(patsubst %/src/,%,$(W_DIRS))

.PHONY: all build_docker build_wasm

all: build_docker build_wasm


# Pattern rule for building docker images
build_docker: $(DIR_NAMES)
	@for dir in $^; do \
		echo "#!/bin/sh" > $$dir/run.sh; \
		echo "/usr/local/bin/$$dir" >> $$dir/run.sh; \
		chmod +x $$dir/run.sh; \
		docker build --platform linux/amd64 -t nebula-function-$$dir --build-arg PROGRAM_NAME=$$dir -f Dockerfile . || true; \
		docker save nebula-function-$$dir | gzip > $(DOCKER_HOME_PATH)/$$dir.tar.gz; \
		rm $$dir/run.sh; \
  done


# Pattern rule for building wasm modules
build_wasm: $(DIR_NAMES)
	@for dir in $^; do \
		cd $$dir; \
		cargo build --release --target wasm32-wasi; \
		cp target/wasm32-wasi/release/$$dir.wasm $(WASM_HOME_PATH)/; \
		cd ..; \
	done

deploy: deploy_docker deploy_wasm

deploy_docker: build_docker 
	@if [ "$(SERVER_USER)" = "default_user" ] || [ "$(SERVER_IP)" = "default_ip" ]; then \
		echo "Please set SERVER_USER and SERVER_IP before deploying"; \
		exit 1; \
	fi
	@for dir in $(DIR_NAMES); do \
		echo "Deploying Docker image for $$dir"; \
		scp $(DOCKER_HOME_PATH)/$$dir.tar.gz $(SERVER_USER)@$(SERVER_IP):$(DOCKER_DEST_PATH)/; \
		echo "Executing SSH command to load Docker image for $$dir"; \
		ssh $(SERVER_USER)@$(SERVER_IP) "sudo docker rmi nebula-function-$$dir || true; sudo docker load -i $(DOCKER_DEST_PATH)/$$dir.tar.gz;"; \
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
