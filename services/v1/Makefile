FUNCTIONS := on-connect on-disconnect s3-presigned-url generate-code send-code send-photo
STACK_NAME := rust-doorbell
ARCH := aarch64-unknown-linux-gnu
ARCH_SPLIT = $(subst -, ,$(ARCH))

build:
ifeq ("$(shell zig targets | jq -r .native.cpu.arch)-$(shell zig targets | jq -r .native.os)-$(shell zig targets | jq -r .native.abi)", "$(word 1,$(ARCH_SPLIT))-$(word 3,$(ARCH_SPLIT))-$(word 4,$(ARCH_SPLIT))")
	@echo "Same host and target. Using native build"
	cargo build --release --target $(ARCH)
else
	@echo "Different host and target. Using zigbuild"
	cargo zigbuild --release --target $(ARCH)
endif

	rm -rf ./build
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ../../target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3 							--template-file ./deployment/s3-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-eventbridge 		--template-file ./deployment/eventbridge-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-websocket 			--template-file ./deployment/websocket-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3presignedurl 	--template-file ./deployment/s3presignedurl-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-stepfunction		--template-file ./deployment/stepfunction-template.yml
	

delete:
	sam delete --profile test --stack-name ${STACK_NAME}-stepfunction
	sam delete --profile test --stack-name ${STACK_NAME}-s3presignedurl
	sam delete --profile test --stack-name ${STACK_NAME}-websocket
	sam delete --profile test --stack-name ${STACK_NAME}-eventbridge
	sam delete --profile test --stack-name ${STACK_NAME}-s3