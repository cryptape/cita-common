// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// This file is generated by rust-protobuf 2.0.4. Do not edit
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
pub struct GetBlockTxn {
    // message fields
    pub block_hash: ::std::vec::Vec<u8>,
    pub short_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl GetBlockTxn {
    pub fn new() -> GetBlockTxn {
        ::std::default::Default::default()
    }

    // bytes block_hash = 1;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_hash, ::std::vec::Vec::new())
    }

    pub fn get_block_hash(&self) -> &[u8] {
        &self.block_hash
    }

    // repeated bytes short_ids = 2;

    pub fn clear_short_ids(&mut self) {
        self.short_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_short_ids(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.short_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_short_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.short_ids
    }

    // Take field
    pub fn take_short_ids(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.short_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_short_ids(&self) -> &[::std::vec::Vec<u8>] {
        &self.short_ids
    }
}

impl ::protobuf::Message for GetBlockTxn {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.short_ids)?;
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
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block_hash);
        }
        for value in &self.short_ids {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block_hash.is_empty() {
            os.write_bytes(1, &self.block_hash)?;
        }
        for v in &self.short_ids {
            os.write_bytes(2, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetBlockTxn {
        GetBlockTxn::new()
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
                    "block_hash",
                    |m: &GetBlockTxn| { &m.block_hash },
                    |m: &mut GetBlockTxn| { &mut m.block_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "short_ids",
                    |m: &GetBlockTxn| { &m.short_ids },
                    |m: &mut GetBlockTxn| { &mut m.short_ids },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockTxn>(
                    "GetBlockTxn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetBlockTxn {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockTxn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockTxn,
        };
        unsafe {
            instance.get(GetBlockTxn::new)
        }
    }
}

impl ::protobuf::Clear for GetBlockTxn {
    fn clear(&mut self) {
        self.clear_block_hash();
        self.clear_short_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockTxn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockTxn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockTxn {
    // message fields
    pub block_hash: ::std::vec::Vec<u8>,
    pub transactions: ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl BlockTxn {
    pub fn new() -> BlockTxn {
        ::std::default::Default::default()
    }

    // bytes block_hash = 1;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_hash, ::std::vec::Vec::new())
    }

    pub fn get_block_hash(&self) -> &[u8] {
        &self.block_hash
    }

    // repeated .UnverifiedTransaction transactions = 2;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[super::blockchain::UnverifiedTransaction] {
        &self.transactions
    }
}

impl ::protobuf::Message for BlockTxn {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block_hash);
        }
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block_hash.is_empty() {
            os.write_bytes(1, &self.block_hash)?;
        }
        for v in &self.transactions {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BlockTxn {
        BlockTxn::new()
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
                    "block_hash",
                    |m: &BlockTxn| { &m.block_hash },
                    |m: &mut BlockTxn| { &mut m.block_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::UnverifiedTransaction>>(
                    "transactions",
                    |m: &BlockTxn| { &m.transactions },
                    |m: &mut BlockTxn| { &mut m.transactions },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockTxn>(
                    "BlockTxn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockTxn {
        static mut instance: ::protobuf::lazy::Lazy<BlockTxn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockTxn,
        };
        unsafe {
            instance.get(BlockTxn::new)
        }
    }
}

impl ::protobuf::Clear for BlockTxn {
    fn clear(&mut self) {
        self.clear_block_hash();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockTxn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTxn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13compact_block.proto\x1a\x10blockchain.proto\"I\n\x0bGetBlockTxn\
    \x12\x1d\n\nblock_hash\x18\x01\x20\x01(\x0cR\tblockHash\x12\x1b\n\tshort\
    _ids\x18\x02\x20\x03(\x0cR\x08shortIds\"e\n\x08BlockTxn\x12\x1d\n\nblock\
    _hash\x18\x01\x20\x01(\x0cR\tblockHash\x12:\n\x0ctransactions\x18\x02\
    \x20\x03(\x0b2\x16.UnverifiedTransactionR\x0ctransactionsJ\xe3\x02\n\x06\
    \x12\x04\0\0\x0c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\
    \x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x04\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x19\n\r\
    \n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x15\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\n\x14\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x17\x18\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x06\x04!\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x06\x04\x0c\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\r\x12\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x06\x13\x1c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x1f\
    \x20\n\n\n\x02\x04\x01\x12\x04\t\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \t\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x04\x19\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\n\x04\t\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\n\
    \x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\n\x14\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\n\x17\x18\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0b\x04\
    4\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\
    \x01\x02\x01\x06\x12\x03\x0b\r\"\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\
    \x0b#/\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0b23b\x06proto3\
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