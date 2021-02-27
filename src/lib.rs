#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const CANARD_MTU_CAN_CLASSIC: size_t = 8;
pub const CANARD_MTU_CAN_FD: size_t = 64;
pub const CANARD_NODE_ID_MAX: CanardNodeID = 127;
pub const CANARD_NODE_ID_UNSET: CanardNodeID = 255;
pub const CANARD_PRIORITY_MAX: CanardPriority = CanardPriority::CanardPriorityOptional;
pub const CANARD_TRANSFER_ID_MAX: CanardTransferID = 31;
