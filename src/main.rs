mod hello;

use hello::say_server::{Say, SayServer};
use hello::area_server::{Area, AreaServer};
use hello::{SayResponse, SayRequest, AreaRequest, AreaResponse};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Clone)]
struct ApiKey(String);

fn api_key_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    const EXPECTED_API_KEY: &str = "super_secret_key";

    let api_key_header = req.metadata().get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    match api_key_header {
        Some(ref s) if s == EXPECTED_API_KEY => {
            req.extensions_mut().insert(ApiKey(s.into()));
            Ok(req)
        }
        Some(_) => Err(Status::unauthenticated("invalid api key")),
        None => Err(Status::unauthenticated("missing api key")),
    }
}

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {    
    async fn send(&self,request:Request<SayRequest>) -> Result<Response<SayResponse>,Status>{
        Ok(Response::new(SayResponse{
             message:format!("hello {}",request.get_ref().name),
        }))
    }
}

#[derive(Default)]
pub struct  MyArea {}
#[tonic::async_trait]
impl Area for MyArea {
    async fn send(&self, request: Request<AreaRequest>) -> Result<Response<AreaResponse>, Status> {
        let req = request.get_ref();
        let result = req.x * req.y;
        Ok(Response::new(AreaResponse { result }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    
    let say = MySay::default();
    let area = MyArea::default();

    println!("Server listening on {}", addr);
    Server::builder()
            .add_service(SayServer::with_interceptor(say, api_key_interceptor))
            .add_service(AreaServer::new(area))
            .serve(addr)
            .await?;
        Ok(())
}