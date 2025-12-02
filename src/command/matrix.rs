use crate::{
    rpc::{self},
    Result,
};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SendMatrixMessageArgs {
    pub room_id: String,
    pub message: String,
}

pub fn send_matrix_message(args: &SendMatrixMessageArgs) -> Result<()> {
    let params = json!({
        "room_id": args.room_id,
        "message": args.message,
    });
    rpc::call("send_matrix_message", params)?.print()
}
