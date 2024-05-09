use autocxx::include_cpp;
use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;

include_cpp! {
    #include "uhal/uhal.hpp"
    #include "valword32.hpp"
    name!(autoffi)
    safety!(unsafe)
    concrete!("uhal::ValWord<uint32_t>", ValWord32)
    concrete!("uhal::ValVector<uint32_t>", ValVector32)
    concrete!("std::vector<uint32_t>", Vector32)
    generate!("uhal::Node")
    generate!("uhal::ConnectionManager")
    generate!("uhal::HwInterface")
    generate!("uhal::ValHeader")
    generate!("uhal::defs::NodePermission")
    generate!("ready")
    generate!("value")
    generate!("ready_valvec")
    generate!("value_valvec")
}

#[cxx::bridge(namespace = "uhal")]
pub mod rawbind {
    unsafe extern "C++" {
        include!("uhal/uhal.hpp");

        type HwInterface = crate::ffi::HwInterface;
        type Node = crate::ffi::Node;

        #[rust_name = "get_node"]
        fn getNode<'a>(self: &'a HwInterface, aId: &CxxString) -> Result<&'a Node>;

        #[rust_name = "disable_logging"]
        fn disableLogging();
    }
}

#[cxx::bridge(namespace = "uhal")]
pub mod resultbind {

    unsafe extern "C++" {
        include!("../extra-cpp/result.hpp");

        type ConnectionManager = crate::ffi::ConnectionManager;
        type HwInterface = crate::ffi::HwInterface;
        type Node = crate::ffi::Node;

        fn new_connection_manager_result(str: &CxxString) -> Result<UniquePtr<ConnectionManager>>;
        fn get_device_from_connection_manager_result(
            cm: &mut UniquePtr<ConnectionManager>,
            string: &CxxString,
        ) -> Result<UniquePtr<HwInterface>>;

        fn get_node_from_node<'a>(node: &'a Node, string: &CxxString) -> Result<&'a Node>;
        fn dispatch_from_node(node: &Node);

        unsafe fn read_block_from_node(node: &Node, size: u32)
            -> Result<UniquePtr<CxxVector<u32>>>;
        unsafe fn write_block_from_node(node: &Node, ptr: *const u32, len: u32) -> Result<()>;
    }
}

pub(crate) use autoffi::ready;
pub(crate) use autoffi::ready_valvec;
pub(crate) use autoffi::uhal::defs::NodePermission;
pub(crate) use autoffi::uhal::ConnectionManager;
pub(crate) use autoffi::uhal::HwInterface;
pub(crate) use autoffi::uhal::Node;
pub(crate) use autoffi::uhal::ValHeader;
pub(crate) use autoffi::value;
pub(crate) use autoffi::value_valvec;
pub(crate) use autoffi::ValVector32;
pub(crate) use autoffi::ValWord32;
