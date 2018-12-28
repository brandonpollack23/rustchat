// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CHAT_ENDPOINT_SEND_MESSAGE: ::grpcio::Method<super::messenger::Message, super::messenger::MessageAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/chat_protocol.ChatEndpoint/SendMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ChatEndpointClient {
    client: ::grpcio::Client,
}

impl ChatEndpointClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ChatEndpointClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_message_opt(&self, req: &super::messenger::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messenger::MessageAck> {
        self.client.unary_call(&METHOD_CHAT_ENDPOINT_SEND_MESSAGE, req, opt)
    }

    pub fn send_message(&self, req: &super::messenger::Message) -> ::grpcio::Result<super::messenger::MessageAck> {
        self.send_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_message_async_opt(&self, req: &super::messenger::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messenger::MessageAck>> {
        self.client.unary_call_async(&METHOD_CHAT_ENDPOINT_SEND_MESSAGE, req, opt)
    }

    pub fn send_message_async(&self, req: &super::messenger::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messenger::MessageAck>> {
        self.send_message_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ChatEndpoint {
    fn send_message(&mut self, ctx: ::grpcio::RpcContext, req: super::messenger::Message, sink: ::grpcio::UnarySink<super::messenger::MessageAck>);
}

pub fn create_chat_endpoint<S: ChatEndpoint + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAT_ENDPOINT_SEND_MESSAGE, move |ctx, req, resp| {
        instance.send_message(ctx, req, resp)
    });
    builder.build()
}
