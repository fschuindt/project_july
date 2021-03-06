// This file is generated by rust-protobuf 3.0.0. Do not edit
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Band {
    // message fields
    pub uid: ::std::string::String,
    pub key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Band {
    pub fn new() -> Band {
        ::std::default::Default::default()
    }

    // string uid = 1;

    pub fn get_uid(&self) -> &str {
        &self.uid
    }

    pub fn clear_uid(&mut self) {
        self.uid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uid(&mut self) -> &mut ::std::string::String {
        &mut self.uid
    }

    // Take field
    pub fn take_uid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uid, ::std::string::String::new())
    }

    // string key = 2;

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Band {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.uid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.uid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.uid);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uid.is_empty() {
            os.write_string(1, &self.uid)?;
        }
        if !self.key.is_empty() {
            os.write_string(2, &self.key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Band {
        Band::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uid",
                    |m: &Band| { &m.uid },
                    |m: &mut Band| { &mut m.uid },
                ));
                fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    |m: &Band| { &m.key },
                    |m: &mut Band| { &mut m.key },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Band>(
                    "Band",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Band {
        static mut instance: ::protobuf::lazy::Lazy<Band> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Band,
        };
        unsafe {
            instance.get(Band::new)
        }
    }
}

impl ::protobuf::Clear for Band {
    fn clear(&mut self) {
        self.uid.clear();
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Band {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Band {
}

#[derive(PartialEq,Clone,Default)]
pub struct TuneRequest {
    // message fields
    pub band: ::protobuf::SingularPtrField<Band>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl TuneRequest {
    pub fn new() -> TuneRequest {
        ::std::default::Default::default()
    }

    // .Streamer.Band band = 1;

    pub fn get_band(&self) -> &Band {
        self.band.as_ref().unwrap_or_else(|| Band::default_instance())
    }

    pub fn clear_band(&mut self) {
        self.band.clear();
    }

    pub fn has_band(&self) -> bool {
        self.band.is_some()
    }

    // Param is passed by value, moved
    pub fn set_band(&mut self, v: Band) {
        self.band = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_band(&mut self) -> &mut Band {
        if self.band.is_none() {
            self.band.set_default();
        }
        self.band.as_mut().unwrap()
    }

    // Take field
    pub fn take_band(&mut self) -> Band {
        self.band.take().unwrap_or_else(|| Band::new())
    }
}

impl ::protobuf::Message for TuneRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.band {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into::<Band, _>(wire_type, is, &mut self.band)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.band.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.band.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TuneRequest {
        TuneRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::rt::make_option_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Band>, _>(
                    "band",
                    |m: &TuneRequest| { &m.band },
                    |m: &mut TuneRequest| { &mut m.band },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TuneRequest>(
                    "TuneRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TuneRequest {
        static mut instance: ::protobuf::lazy::Lazy<TuneRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TuneRequest,
        };
        unsafe {
            instance.get(TuneRequest::new)
        }
    }
}

impl ::protobuf::Clear for TuneRequest {
    fn clear(&mut self) {
        self.band.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TuneRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TuneRequest {
}

#[derive(PartialEq,Clone,Default)]
pub struct Video {
    // message fields
    pub index: ::std::string::String,
    pub chunk: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Video {
    pub fn new() -> Video {
        ::std::default::Default::default()
    }

    // string index = 1;

    pub fn get_index(&self) -> &str {
        &self.index
    }

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: ::std::string::String) {
        self.index = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index(&mut self) -> &mut ::std::string::String {
        &mut self.index
    }

    // Take field
    pub fn take_index(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.index, ::std::string::String::new())
    }

    // bytes chunk = 2;

    pub fn get_chunk(&self) -> &[u8] {
        &self.chunk
    }

    pub fn clear_chunk(&mut self) {
        self.chunk.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunk(&mut self, v: ::std::vec::Vec<u8>) {
        self.chunk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chunk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.chunk
    }

    // Take field
    pub fn take_chunk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.chunk, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Video {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.index)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.chunk)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.index.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.index);
        }
        if !self.chunk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.chunk);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.index.is_empty() {
            os.write_string(1, &self.index)?;
        }
        if !self.chunk.is_empty() {
            os.write_bytes(2, &self.chunk)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Video {
        Video::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "index",
                    |m: &Video| { &m.index },
                    |m: &mut Video| { &mut m.index },
                ));
                fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "chunk",
                    |m: &Video| { &m.chunk },
                    |m: &mut Video| { &mut m.chunk },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Video>(
                    "Video",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Video {
        static mut instance: ::protobuf::lazy::Lazy<Video> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Video,
        };
        unsafe {
            instance.get(Video::new)
        }
    }
}

impl ::protobuf::Clear for Video {
    fn clear(&mut self) {
        self.index.clear();
        self.chunk.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Video {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Video {
}

#[derive(PartialEq,Clone,Default)]
pub struct BroadcastResponse {
    // message fields
    pub success: bool,
    pub band: ::protobuf::SingularPtrField<Band>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl BroadcastResponse {
    pub fn new() -> BroadcastResponse {
        ::std::default::Default::default()
    }

    // bool success = 1;

    pub fn get_success(&self) -> bool {
        self.success
    }

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    // .Streamer.Band band = 2;

    pub fn get_band(&self) -> &Band {
        self.band.as_ref().unwrap_or_else(|| Band::default_instance())
    }

    pub fn clear_band(&mut self) {
        self.band.clear();
    }

    pub fn has_band(&self) -> bool {
        self.band.is_some()
    }

    // Param is passed by value, moved
    pub fn set_band(&mut self, v: Band) {
        self.band = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_band(&mut self) -> &mut Band {
        if self.band.is_none() {
            self.band.set_default();
        }
        self.band.as_mut().unwrap()
    }

    // Take field
    pub fn take_band(&mut self) -> Band {
        self.band.take().unwrap_or_else(|| Band::new())
    }
}

impl ::protobuf::Message for BroadcastResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.band {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into::<Band, _>(wire_type, is, &mut self.band)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success != false {
            my_size += 2;
        }
        if let Some(ref v) = self.band.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.success != false {
            os.write_bool(1, self.success)?;
        }
        if let Some(ref v) = self.band.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BroadcastResponse {
        BroadcastResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    |m: &BroadcastResponse| { &m.success },
                    |m: &mut BroadcastResponse| { &mut m.success },
                ));
                fields.push(::protobuf::reflect::rt::make_option_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Band>, _>(
                    "band",
                    |m: &BroadcastResponse| { &m.band },
                    |m: &mut BroadcastResponse| { &mut m.band },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BroadcastResponse>(
                    "BroadcastResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BroadcastResponse {
        static mut instance: ::protobuf::lazy::Lazy<BroadcastResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BroadcastResponse,
        };
        unsafe {
            instance.get(BroadcastResponse::new)
        }
    }
}

impl ::protobuf::Clear for BroadcastResponse {
    fn clear(&mut self) {
        self.success = false;
        self.band.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BroadcastResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BroadcastResponse {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0csyrinx.proto\x12\x08Streamer\"*\n\x04Band\x12\x10\n\x03uid\x18\x01\
    \x20\x01(\tR\x03uid\x12\x10\n\x03key\x18\x02\x20\x01(\tR\x03key\"1\n\x0b\
    TuneRequest\x12\"\n\x04band\x18\x01\x20\x01(\x0b2\x0e.Streamer.BandR\x04\
    band\"3\n\x05Video\x12\x14\n\x05index\x18\x01\x20\x01(\tR\x05index\x12\
    \x14\n\x05chunk\x18\x02\x20\x01(\x0cR\x05chunk\"Q\n\x11BroadcastResponse\
    \x12\x18\n\x07success\x18\x01\x20\x01(\x08R\x07success\x12\"\n\x04band\
    \x18\x02\x20\x01(\x0b2\x0e.Streamer.BandR\x04band2J\n\tBroadcast\x12=\n\
    \tBroadcast\x12\x0f.Streamer.Video\x1a\x1b.Streamer.BroadcastResponse\"\
    \0(\x012:\n\x04Tune\x122\n\x04Tune\x12\x15.Streamer.TuneRequest\x1a\x0f.\
    Streamer.Video\"\00\x01J\xa0\x06\n\x06\x12\x04\0\0\x1d\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x10\n\n\n\x02\x06\0\
    \x12\x04\x04\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\x08\x11\n\x0b\n\
    \x04\x06\0\x02\0\x12\x03\x05\x02=\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x05\x06\x0f\n\x0c\n\x05\x06\0\x02\0\x05\x12\x03\x05\x11\x17\n\x0c\n\x05\
    \x06\0\x02\0\x02\x12\x03\x05\x18\x1d\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\
    \x05(9\n\n\n\x02\x06\x01\x12\x04\x08\0\n\x01\n\n\n\x03\x06\x01\x01\x12\
    \x03\x08\x08\x0c\n\x0b\n\x04\x06\x01\x02\0\x12\x03\t\x022\n\x0c\n\x05\
    \x06\x01\x02\0\x01\x12\x03\t\x06\n\n\x0c\n\x05\x06\x01\x02\0\x02\x12\x03\
    \t\x0c\x17\n\x0c\n\x05\x06\x01\x02\0\x06\x12\x03\t\"(\n\x0c\n\x05\x06\
    \x01\x02\0\x03\x12\x03\t).\n\n\n\x02\x04\0\x12\x04\x0c\0\x0f\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x0c\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\r\x02\
    \x11\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\x02\x0c\x0e\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\t\x0c\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x0f\x10\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x0e\x02\x11\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0e\x02\r\x11\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x0e\t\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0e\x0f\
    \x10\n\n\n\x02\x04\x01\x12\x04\x11\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x11\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x12\x02\x10\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x12\x02\x11\x15\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03\x12\x02\x06\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x12\x07\x0b\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x12\x0e\x0f\n\n\n\x02\x04\x02\x12\
    \x04\x15\0\x18\x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\x08\r\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\x16\x02\x13\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\
    \x16\x02\x15\x0f\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x16\x02\x08\n\x0c\
    \n\x05\x04\x02\x02\0\x01\x12\x03\x16\t\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x16\x11\x12\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x17\x02\x12\n\r\
    \n\x05\x04\x02\x02\x01\x04\x12\x04\x17\x02\x16\x13\n\x0c\n\x05\x04\x02\
    \x02\x01\x05\x12\x03\x17\x02\x07\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\
    \x17\x08\r\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x17\x10\x11\n\n\n\x02\
    \x04\x03\x12\x04\x1a\0\x1d\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1a\x08\x19\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1b\x02\x13\n\r\n\x05\x04\x03\x02\0\
    \x04\x12\x04\x1b\x02\x1a\x1b\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1b\
    \x02\x06\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1b\x07\x0e\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03\x1b\x11\x12\n\x0b\n\x04\x04\x03\x02\x01\x12\
    \x03\x1c\x02\x10\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x1c\x02\x1b\x13\n\
    \x0c\n\x05\x04\x03\x02\x01\x06\x12\x03\x1c\x02\x06\n\x0c\n\x05\x04\x03\
    \x02\x01\x01\x12\x03\x1c\x07\x0b\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\
    \x1c\x0e\x0fb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
