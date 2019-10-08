// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `numproto/protobuf/ndarray.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct NDArray {
    // message fields
    pub ndarray: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NDArray {
    fn default() -> &'a NDArray {
        <NDArray as ::protobuf::Message>::default_instance()
    }
}

impl NDArray {
    pub fn new() -> NDArray {
        ::std::default::Default::default()
    }

    // bytes ndarray = 1;


    pub fn get_ndarray(&self) -> &[u8] {
        &self.ndarray
    }
    pub fn clear_ndarray(&mut self) {
        self.ndarray.clear();
    }

    // Param is passed by value, moved
    pub fn set_ndarray(&mut self, v: ::std::vec::Vec<u8>) {
        self.ndarray = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ndarray(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ndarray
    }

    // Take field
    pub fn take_ndarray(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.ndarray, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for NDArray {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.ndarray)?;
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
        if !self.ndarray.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.ndarray);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.ndarray.is_empty() {
            os.write_bytes(1, &self.ndarray)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NDArray {
        NDArray::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ndarray",
                    |m: &NDArray| { &m.ndarray },
                    |m: &mut NDArray| { &mut m.ndarray },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NDArray>(
                    "NDArray",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static NDArray {
        static mut instance: ::protobuf::lazy::Lazy<NDArray> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NDArray,
        };
        unsafe {
            instance.get(NDArray::new)
        }
    }
}

impl ::protobuf::Clear for NDArray {
    fn clear(&mut self) {
        self.ndarray.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NDArray {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NDArray {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fnumproto/protobuf/ndarray.proto\x12\x11numproto.protobuf\"#\n\x07N\
    DArray\x12\x18\n\x07ndarray\x18\x01\x20\x01(\x0cR\x07ndarrayJz\n\x06\x12\
    \x04\0\0\x06\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\x08\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x04\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x16\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x11\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\n\x11\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x05\x14\x15b\x06proto3\
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