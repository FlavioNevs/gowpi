use crate::context::Context;

pub async fn test_handler(_ctx: Context) -> String {
    format!("test called, state_thing was: nhaa")
}