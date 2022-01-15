FUNCTIONS := on-connect on-disconnect
STACK_NAME := rust-doorbell
ARCH := aarch64-unknown-linux-gnu

build:
	rm -rf ./build
	mkdir -p ./build
	cross build --release --target $(ARCH)
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	#sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3 --template-file ./infrastructure/s3-template.yml  
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-websocket --template-file ./infrastructure/websocket-template.yml

delete:
	#sam delete --profile test --stack-name ${STACK_NAME}-s3
	sam delete --profile test --stack-name ${STACK_NAME}-websocket