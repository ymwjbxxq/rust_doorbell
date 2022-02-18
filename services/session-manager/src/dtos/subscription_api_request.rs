use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptiontApiRequest {
    pub body: String,
    pub request_context: RequestContext,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub api_id: String,
    pub domain_prefix: String,
    pub domain_name: String,
    pub stage: String,
}


// {
//    "body":{
//       "prop1":"ciao"
//    },
//    "headers":null,
//    "httpMethod":"POST",
//    "isBase64Encoded":false,
//    "multiValueHeaders":null,
//    "multiValueQueryStringParameters":null,
//    "path":"/subscription",
//    "pathParameters":null,
//    "queryStringParameters":null,
//    "requestContext":{
//       "accountId":"xxxxx",
//       "apiId":"xxxxx",
//       "domainName":"testPrefix.testDomainName",
//       "domainPrefix":"testPrefix",
//       "extendedRequestId":"NvWWKEZbliAFliA=",
//       "httpMethod":"POST",
//       "identity":{
//          "accessKey":"xxxxx",
//          "accountId":"xxxxx",
//          "apiKey":"test-invoke-api-key",
//          "apiKeyId":"test-invoke-api-key-id",
//          "caller":"xxxxx:xxxxx",
//          "cognitoAuthenticationProvider":null,
//          "cognitoAuthenticationType":null,
//          "cognitoIdentityId":null,
//          "cognitoIdentityPoolId":null,
//          "principalOrgId":null,
//          "sourceIp":"test-invoke-source-ip",
//          "user":"xxxxx:xxxxx",
//          "userAgent":"aws-internal/3 aws-sdk-java/1.12.154 Linux/5.4.156-94.273.amzn2int.x86_64 OpenJDK_64-Bit_Server_VM/25.322-b06 java/1.8.0_322 vendor/Oracle_Corporation cfg/retry-mode/standard",
//          "userArn":"arn:aws:sts::xxxxx:assumed-role/xxxxx/xxxxx"
//       },
//       "path":"/subscription",
//       "protocol":"HTTP/1.1",
//       "requestId":"e5488776-afe4-4e5e-92b1-37bd23f234d6",
//       "requestTime":"18/Feb/2022:13:23:12 +0000",
//       "requestTimeEpoch":1645190592806,
//       "resourceId":"ddw8yd",
//       "resourcePath":"/subscription",
//       "stage":"test-invoke-stage"
//    },
//    "resource":"/subscription",
//    "stageVariables":null
// }
