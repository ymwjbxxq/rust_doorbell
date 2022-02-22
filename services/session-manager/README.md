# Session Manager

You can read the full article [here]()

"Project Doorbell & Co." is selling different types of surveillance devices like:

- Doorbell
- Camera internal and external
- Smoke and CO alarm
- Many others

As with every company, there are subscriptions plans:

- Free
- Pro

On this repository, I take care of a Free plan.


## Use cases ##

- A free user cannot watch more than two streams at any time.
- A free user cannot have more than three registered devices

## Solution ##

Session manager service will store user subscriptions.

| id | Type | Streams | Devices |
| ---|---|---|---|
| user 1 | free | 2 | 3|
| user 2 | pro | 100 | 100 |

If the user1 register three devices, the row will be updated as the following:

| id | Type | Streams | Devices |
| ---|---|---|---|
| user 1 | free | 2 | 0|

When the user registers the 4th device, the API will return the error asking the user to move to the Pro plan.

The same applies to the concurrent stream that the user is watching with the application. Again, if it reaches the free allowances, the API will return the error asking the user to move to the Pro plan.

## Swagger ##

[Swagger UI](https://swagger.io/tools/swagger-ui/) allows anyone — be it your development team or your end consumers — to visualize and interact with the API's resources without having any of the implementation logic in place. Instead, it's automatically generated from your OpenAPI (formerly known as Swagger) Specification, with the visual documentation making it easy for back end implementation and client-side consumption.

In the dist folder, I have downloaded version 4.0, and I already put under the /doc folder the JSON file needed for the UI.

If you want to do it, the steps are:

1. Create a public AWS S3 bucket and configure it as [Static website hosting](https://docs.aws.amazon.com/AmazonS3/latest/userguide/WebsiteHosting.html)
2. Download swagger UI
3. Use only the /dist folder. The folder includes all the HTML, CSS and JS files needed to run SwaggerUI on a static website
3. Create inside the swagger-ui folder /docs
4. Add your API endpoint documents
5. Add inside the /dist folder a doc_list.json file
6. Change the index.hml to use the doc_list.json file
7. Sync the /dist folder with S3 (aws s3 sync . s3://[BUCKET_NAME] )
8. Access swagger UI https://[BUCKET_NAME].s3.[REGION].amazonaws.com/index.html

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/services/session-manager/readme/swagger.png)

For this repository example, the doc_list.json is made of:
```
[
  {
    "url": "docs/device.json",
    "name": "Device API"
  },
  {
    "url": "docs/stream.json",
    "name": "Stream API"
  },
  {
    "url": "docs/subscription.json",
    "name": "Subscription API"
  }
]
```

The index.html file should contain this javascript script to load the API definition.
```
window.onload = function() {
  var allText;
  function readTextFile(file) {
    var rawFile = new XMLHttpRequest();
    rawFile.open("GET", file, false);
    rawFile.onreadystatechange = function () {
      if (rawFile.readyState === 4) {
        if (rawFile.status === 200 || rawFile.status == 0) {
          allText = rawFile.responseText;
          console.log(allText)
        }
      }
    }
    rawFile.send(null);
  }

  readTextFile("./docs_list.json") 
  // Begin Swagger UI call region
  const ui = SwaggerUIBundle({
    urls: JSON.parse(allText),
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "StandaloneLayout"
  });
  // End Swagger UI call region

  window.ui = ui;
};
```

## Requirements

* [Create an AWS account](https://portal.aws.amazon.com/gp/aws/developer/registration/index.html) if you do not already have one and log in. The IAM user that you use must have sufficient permissions to make necessary AWS service calls and manage AWS resources.
* [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) installed and configured
* [Git Installed](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* [AWS Serverless Application Model](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) (AWS SAM) installed
* [Rust](https://www.rust-lang.org/) 1.56.0 or higher
* [cargo-zigbuild](https://github.com/messense/cargo-zigbuild) and [Zig](https://ziglang.org/) for cross-compilation

### Deployment Instructions ###

1. Install dependencies and build:
    ```
    make build
    ```
2. From the command line, use AWS SAM to deploy the AWS resources for the pattern as specified in the template.yml file:
    ```
    make deploy
    ```
3. During the prompts:
    * Enter a stack name
    * Enter the desired AWS Region
    * Allow SAM CLI to create IAM roles with the required permissions.

    Once you have run `sam deploy -guided` mode once and saved arguments to a configuration file (samconfig.toml), you can use `sam deploy` in future to use these defaults.

4. Note the outputs from the SAM deployment process. These contain the resource names and/or ARNs used for testing.


### Testing ###

In theory this project should be exposed by a PRIVATE API and so in the template there is
```
      # EndpointConfiguration:
      #   Type: PRIVATE
      #   VPCEndpointIds: 
      #     - !Ref VpcId
```

To test the a Private API the steps are:

1. Create an EC2 
2. Install curl

Run the following:
```
curl -X POST -H "content-type:application/json" https://{api_id}.execute-api.{region}.amazonaws.com/{stage}/subscription -d '{"user_id": "3","plan_id": "free","streams": 2, "devices": 3}'
curl -X POST -H "content-type:application/json" https://{api_id}.execute-api.{region}.amazonaws.com/{stage}/device -d '{"user_id": "3","device_count": 1}'
curl -X POST -H "content-type:application/json" https://{api_id}.execute-api.{region}.amazonaws.com/{stage}/stream -d '{"user_id": "3","video_id": "1"}'
```

### Cleanup ###

1. Delete the stack
    "`bash
    make delete
    ```
2. Confirm the stack has been deleted
    ```bash
    aws cloudformation list-stacks --query "StackSummaries[?contains(StackName,'STACK_NAME')].StackStatus"
    ```