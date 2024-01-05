mod foo {
    tonic::include_proto!("foo");
}

use foo::{foo_server::{Foo, FooServer}, foo_client::FooClient, Req, Res};
use tonic::{transport::Server, transport::Channel, Request, Response, Status};

pub struct FooImpl;

#[tonic::async_trait]
impl Foo for FooImpl {
    async fn connect(
        &self,
        request: Request<Req>,
    ) -> Result<Response<Res>, Status> {
        Ok(Response::new(foo::Res{s: request.get_ref().s.chars().rev().collect::<_>() }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = FooImpl{};

    let server = Server::builder()
        .add_service(FooServer::new(service))
        .serve(addr);
    tokio::spawn(async move {
        server.await.unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let ch = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = FooClient::new(ch);
    let req = tonic::Request::new(Req {s: "manul".to_owned() });
    println!("sent: {}", &req.get_ref().s);
    let res = client.connect(req).await?;
    println!("got: {}", &res.get_ref().s);

    Ok(())
}
