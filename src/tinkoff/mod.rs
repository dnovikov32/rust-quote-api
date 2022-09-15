pub mod proto;

use std::error::Error;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use tonic::transport::Channel;
use proto::instruments_service_client::InstrumentsServiceClient;
use proto::InstrumentsRequest;

#[derive(Debug)]
pub struct TinkoffInterceptor {
    token: String,
}

impl Interceptor for TinkoffInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut req = request;

        req.metadata_mut().append(
            "authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );

        Ok(req)
    }
}

#[derive(Debug)]
pub struct TinkoffService {
    host: String,
    token: String,
}

impl TinkoffService {
    pub fn new(host: String, token: String) -> Self {
        Self {
            host,
            token,
        }
    }

    pub async fn create_channel(&self) -> Result<Channel, Box<dyn Error>> {
        let host: String = self.host.to_owned();
        let static_host: &str = Box::leak(host.into_boxed_str());

        let channel = Channel::from_static(static_host)
            .connect()
            .await?;

        Ok(channel)
    }

    pub async fn instruments(
        &self,
        channel: Channel
    ) -> Result<InstrumentsServiceClient<InterceptedService<Channel, TinkoffInterceptor>>, Box<dyn Error>> {

        let client = InstrumentsServiceClient::with_interceptor(
            channel,
            TinkoffInterceptor {
                token: self.token.clone()
            }
        );
        
        Ok(client)
    }

}