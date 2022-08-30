// Read This link 
// https://genekuo.medium.com/coding-a-simple-microservices-with-rust-3fbde8e32adc
use std::error::Error as StdError;

use rmp_serde as rmps; 
use serde::{Deserialize, Serialize};

use rpcx_derive::*;
use rpcx_protocol::{Error, ErrorKind, Result, RpcxParam, SerializeType};

#[derive(RpcxParam, Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ArithAddArgs {
    #[serde(rename = "A")]
    pub a: u64,
    #[serde(rename = "B")]
    pub b: u64,
}

#[derive(RpcxParam, Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ArithAddReply {
    #[serde(rename = "C")]
    pub c: u64,
}

use mul_model::{ArithAddArgs, ArithAddReply};
use rpcx::*;

fn add(args: ArithAddArgs) -> ArithAddReply {
    ArithAddReply { c: args.a + args.b }
}

fn mul(args: ArithAddArgs) -> ArithAddReply {
    ArithAddReply { c: args.a * args.b }
}

fn main() {
    let mut rpc_server = Server::new("127.0.0.1:8972".to_owned());
    register_func!(
        rpc_server,
        "Arith",
        "Add",
        add,
        ArithAddArgs,
        ArithAddReply
    );

    register_func!(
        rpc_server,
        "Arith",
        "Mul",
        mul,
        ArithAddArgs,
        ArithAddReply
    );

    rpc_server.start().unwrap();
}
