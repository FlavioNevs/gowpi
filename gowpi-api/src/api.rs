use crate::{Error, Response, router::Router, context::Context};

use hyper::{
    service::{make_service_fn, service_fn},
    Request, Server
};

use std::sync::Arc;

pub struct Api {
    pub router: Router
}

impl Api {
    pub fn new() -> Self {
        Api {
            router:Router::new()
        }
    }
    pub async fn run<S: Into<String>>(self, address: S, port: S) -> Result<(), Box<dyn std::error::Error>> {

        let shared_router = Arc::new(self.router);
        let new_service = make_service_fn(move |_| {
    
            let router_capture = shared_router.clone();
            async {
                Ok::<_, Error>(service_fn(move |req| {
                    route(router_capture.clone(), req)
                }))
            }
        });
    
        let addr = format!("{}:{}", address.into(), port.into()).parse().expect("address creation works");
        
        let server = Server::bind(&addr).serve(new_service);

        println!("Listening on http://{}", addr);
        let _ = server.await;
    
        Ok(())
    }
}


async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler.handler
        .invoke(Context::new(req, found_handler.params))
        .await;
    Ok(resp)
}