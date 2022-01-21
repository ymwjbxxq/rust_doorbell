FUNCTIONS := on-connect on-disconnect s3-presigned-url
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
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3 							--template-file ./infrastructure/s3-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-stepfunction		--template-file ./infrastructure/stepfunction-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-eventbridge 		--template-file ./infrastructure/eventbridge-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-websocket 			--template-file ./infrastructure/websocket-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3presignedurl 	--template-file ./infrastructure/s3presignedurl-template.yml
	

delete:
	sam delete --profile test --stack-name ${STACK_NAME}-s3presignedurl-template
	sam delete --profile test --stack-name ${STACK_NAME}-websocket
	sam delete --profile test --stack-name ${STACK_NAME}-eventbridge
	sam delete --profile test --stack-name ${STACK_NAME}-stepfunction
	sam delete --profile test --stack-name ${STACK_NAME}-s3