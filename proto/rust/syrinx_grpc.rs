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


// interface

pub trait Broadcast {
    fn broadcast(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::syrinx::Video>) -> ::grpc::SingleResponse<super::syrinx::BroadcastResponse>;
}

// client

pub struct BroadcastClient {
    grpc_client: ::grpc::Client,
    method_Broadcast: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::syrinx::Video, super::syrinx::BroadcastResponse>>,
}

impl BroadcastClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        BroadcastClient {
            grpc_client: grpc_client,
            method_Broadcast: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Streamer.Broadcast/Broadcast".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            BroadcastClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            BroadcastClient::with_client(c)
        })
    }
}

impl Broadcast for BroadcastClient {
    fn broadcast(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::syrinx::Video>) -> ::grpc::SingleResponse<super::syrinx::BroadcastResponse> {
        self.grpc_client.call_client_streaming(o, p, self.method_Broadcast.clone())
    }
}

// server

pub struct BroadcastServer;


impl BroadcastServer {
    pub fn new_service_def<H : Broadcast + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Streamer.Broadcast",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Streamer.Broadcast/Broadcast".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerClientStreaming::new(move |o, p| handler_copy.broadcast(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Tune {
    fn tune(&self, o: ::grpc::RequestOptions, p: super::syrinx::TuneRequest) -> ::grpc::StreamingResponse<super::syrinx::Video>;
}

// client

pub struct TuneClient {
    grpc_client: ::grpc::Client,
    method_Tune: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::syrinx::TuneRequest, super::syrinx::Video>>,
}

impl TuneClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        TuneClient {
            grpc_client: grpc_client,
            method_Tune: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Streamer.Tune/Tune".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            TuneClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            TuneClient::with_client(c)
        })
    }
}

impl Tune for TuneClient {
    fn tune(&self, o: ::grpc::RequestOptions, p: super::syrinx::TuneRequest) -> ::grpc::StreamingResponse<super::syrinx::Video> {
        self.grpc_client.call_server_streaming(o, p, self.method_Tune.clone())
    }
}

// server

pub struct TuneServer;


impl TuneServer {
    pub fn new_service_def<H : Tune + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Streamer.Tune",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Streamer.Tune/Tune".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.tune(o, p))
                    },
                ),
            ],
        )
    }
}
