use hello::say_server::{Say, SayServer};
use hello::area_server::{Area, AreaServer};
use tonic::{transport::Server, Request, Response, Status};
use hello::{SayResponse, SayRequest, AreaRequest, AreaResponse};
mod hello; 

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
            .add_service(SayServer::new(say))
            .add_service(AreaServer::new(area))
            .serve(addr)
            .await?;
        Ok(())
}