#[derive(Debug)]
pub struct AWSConfig {
    config: aws_types::config::Config,
}

impl AWSConfig {
    pub fn set_config(config: aws_types::config::Config) -> Self {
        Self { config }
    }

    pub fn on_connect(&self) -> AWSClient {
        let aws_client = AWSClient {
            dynamo_db_client: Some(self.dynamo_client()),
            event_bridge: Some(self.event_bridge_client()),
            s3_client: None,
        };

        aws_client
    }

    pub fn on_disconnect(&self) -> AWSClient {
        let aws_client = AWSClient {
            dynamo_db_client: Some(self.dynamo_client()),
            event_bridge: None,
            s3_client: None,
        };

        aws_client
    }

    pub fn on_s3_presigned_url(&self) -> AWSClient {
        let aws_client = AWSClient {
            dynamo_db_client: None,
            event_bridge: None,
            s3_client: Some(self.s3_client()),
        };

        aws_client
    }

    fn dynamo_client(&self) -> aws_sdk_dynamodb::Client {
        aws_sdk_dynamodb::Client::new(&self.config)
    }

    fn s3_client(&self) -> aws_sdk_s3::Client {
        aws_sdk_s3::Client::new(&self.config)
    }

    fn event_bridge_client(&self) -> aws_sdk_eventbridge::Client {
        aws_sdk_eventbridge::Client::new(&self.config)
    }
}

#[derive(Clone, Debug)]
pub struct AWSClient {
    pub dynamo_db_client: Option<aws_sdk_dynamodb::Client>,
    pub s3_client: Option<aws_sdk_s3::Client>,
    pub event_bridge: Option<aws_sdk_eventbridge::Client>,
}
