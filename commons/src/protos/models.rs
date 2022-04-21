// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `models.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct GetRequest {
    // message fields
    pub name: ::std::string::String,
    pub age: i32,
    pub features: ::protobuf::RepeatedField<::std::string::String>,
    pub descr: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetRequest {
    fn default() -> &'a GetRequest {
        <GetRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int32 age = 2;


    pub fn get_age(&self) -> i32 {
        self.age
    }
    pub fn clear_age(&mut self) {
        self.age = 0;
    }

    // Param is passed by value, moved
    pub fn set_age(&mut self, v: i32) {
        self.age = v;
    }

    // repeated string features = 3;


    pub fn get_features(&self) -> &[::std::string::String] {
        &self.features
    }
    pub fn clear_features(&mut self) {
        self.features.clear();
    }

    // Param is passed by value, moved
    pub fn set_features(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_features(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.features
    }

    // Take field
    pub fn take_features(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.features, ::protobuf::RepeatedField::new())
    }

    // string descr = 4;


    pub fn get_descr(&self) -> &str {
        &self.descr
    }
    pub fn clear_descr(&mut self) {
        self.descr.clear();
    }

    // Param is passed by value, moved
    pub fn set_descr(&mut self, v: ::std::string::String) {
        self.descr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descr(&mut self) -> &mut ::std::string::String {
        &mut self.descr
    }

    // Take field
    pub fn take_descr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descr, ::std::string::String::new())
    }
}

impl ::protobuf::Message for GetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.age = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.features)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descr)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.age != 0 {
            my_size += ::protobuf::rt::value_size(2, self.age, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.features {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.descr.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.descr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.age != 0 {
            os.write_int32(2, self.age)?;
        }
        for v in &self.features {
            os.write_string(3, &v)?;
        };
        if !self.descr.is_empty() {
            os.write_string(4, &self.descr)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &GetRequest| { &m.name },
                |m: &mut GetRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "age",
                |m: &GetRequest| { &m.age },
                |m: &mut GetRequest| { &mut m.age },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "features",
                |m: &GetRequest| { &m.features },
                |m: &mut GetRequest| { &mut m.features },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "descr",
                |m: &GetRequest| { &m.descr },
                |m: &mut GetRequest| { &mut m.descr },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetRequest>(
                "GetRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetRequest {
        static instance: ::protobuf::rt::LazyV2<GetRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetRequest::new)
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.age = 0;
        self.features.clear();
        self.descr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetResponse {
    // message fields
    pub status: GetResponse_Status,
    pub address: ::std::string::String,
    pub city: ::std::string::String,
    pub zipcode: i32,
    pub ts: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetResponse {
    fn default() -> &'a GetResponse {
        <GetResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    // .GetResponse.Status status = 1;


    pub fn get_status(&self) -> GetResponse_Status {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = GetResponse_Status::OK;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: GetResponse_Status) {
        self.status = v;
    }

    // string address = 2;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string city = 3;


    pub fn get_city(&self) -> &str {
        &self.city
    }
    pub fn clear_city(&mut self) {
        self.city.clear();
    }

    // Param is passed by value, moved
    pub fn set_city(&mut self, v: ::std::string::String) {
        self.city = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_city(&mut self) -> &mut ::std::string::String {
        &mut self.city
    }

    // Take field
    pub fn take_city(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.city, ::std::string::String::new())
    }

    // int32 zipcode = 4;


    pub fn get_zipcode(&self) -> i32 {
        self.zipcode
    }
    pub fn clear_zipcode(&mut self) {
        self.zipcode = 0;
    }

    // Param is passed by value, moved
    pub fn set_zipcode(&mut self, v: i32) {
        self.zipcode = v;
    }

    // .google.protobuf.Timestamp ts = 5;


    pub fn get_ts(&self) -> &::protobuf::well_known_types::Timestamp {
        self.ts.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_ts(&mut self) {
        self.ts.clear();
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.ts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ts(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.ts.is_none() {
            self.ts.set_default();
        }
        self.ts.as_mut().unwrap()
    }

    // Take field
    pub fn take_ts(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.ts.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.ts {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.city)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.zipcode = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ts)?;
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
        if self.status != GetResponse_Status::OK {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.address);
        }
        if !self.city.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.city);
        }
        if self.zipcode != 0 {
            my_size += ::protobuf::rt::value_size(4, self.zipcode, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.status != GetResponse_Status::OK {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.status))?;
        }
        if !self.address.is_empty() {
            os.write_string(2, &self.address)?;
        }
        if !self.city.is_empty() {
            os.write_string(3, &self.city)?;
        }
        if self.zipcode != 0 {
            os.write_int32(4, self.zipcode)?;
        }
        if let Some(ref v) = self.ts.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GetResponse_Status>>(
                "status",
                |m: &GetResponse| { &m.status },
                |m: &mut GetResponse| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &GetResponse| { &m.address },
                |m: &mut GetResponse| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "city",
                |m: &GetResponse| { &m.city },
                |m: &mut GetResponse| { &mut m.city },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "zipcode",
                |m: &GetResponse| { &m.zipcode },
                |m: &mut GetResponse| { &mut m.zipcode },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "ts",
                |m: &GetResponse| { &m.ts },
                |m: &mut GetResponse| { &mut m.ts },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetResponse>(
                "GetResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetResponse {
        static instance: ::protobuf::rt::LazyV2<GetResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetResponse::new)
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.status = GetResponse_Status::OK;
        self.address.clear();
        self.city.clear();
        self.zipcode = 0;
        self.ts.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GetResponse_Status {
    OK = 0,
    ERR = 1,
    NOT_FOUND = 2,
}

impl ::protobuf::ProtobufEnum for GetResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(GetResponse_Status::OK),
            1 => ::std::option::Option::Some(GetResponse_Status::ERR),
            2 => ::std::option::Option::Some(GetResponse_Status::NOT_FOUND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GetResponse_Status] = &[
            GetResponse_Status::OK,
            GetResponse_Status::ERR,
            GetResponse_Status::NOT_FOUND,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<GetResponse_Status>("GetResponse.Status", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for GetResponse_Status {
}

impl ::std::default::Default for GetResponse_Status {
    fn default() -> Self {
        GetResponse_Status::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse_Status {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cmodels.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"n\n\nGetReque\
    st\x12\x14\n\x04name\x18\x01\x20\x01(\tR\x04nameB\0\x12\x12\n\x03age\x18\
    \x02\x20\x01(\x05R\x03ageB\0\x12\x1c\n\x08features\x18\x03\x20\x03(\tR\
    \x08featuresB\0\x12\x16\n\x05descr\x18\x04\x20\x01(\tR\x05descrB\0:\0\"\
    \xe6\x01\n\x0bGetResponse\x12-\n\x06status\x18\x01\x20\x01(\x0e2\x13.Get\
    Response.StatusR\x06statusB\0\x12\x1a\n\x07address\x18\x02\x20\x01(\tR\
    \x07addressB\0\x12\x14\n\x04city\x18\x03\x20\x01(\tR\x04cityB\0\x12\x1a\
    \n\x07zipcode\x18\x04\x20\x01(\x05R\x07zipcodeB\0\x12,\n\x02ts\x18\x05\
    \x20\x01(\x0b2\x1a.google.protobuf.TimestampR\x02tsB\0\"*\n\x06Status\
    \x12\x06\n\x02OK\x10\0\x12\x07\n\x03ERR\x10\x01\x12\r\n\tNOT_FOUND\x10\
    \x02\x1a\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}