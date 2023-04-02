pub mod router;
pub mod context;
pub mod handler;
pub mod api;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;