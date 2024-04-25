pub mod connection;
#[allow(unused_imports)]
pub mod ffi;
pub mod hw;

#[cfg(test)]
mod test {
    use crate::connection::ConnectionManager;

    // #[test]
    // fn test_stuffs() {
    //     let_cxx_string!(
    //         filename = "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml"
    //     );
    //     let_cxx_string!(devicename = "dummy.udp");
    //     let_cxx_string!(node_test = "REG");

    //     let mut connection = ConnectionManager::new1(&filename).within_unique_ptr();
    //     let mut hw = connection
    //         .pin_mut()
    //         .getDevice(&devicename)
    //         .within_unique_ptr();

    //     let hw = hw.pin_mut();

    //     let node = hw.get_node(&node_test);
    //     let value = node.read();

    //     hw.dispatch();

    //     panic!("Value: {}", ffi::value(&value));
    // }

    #[test]
    fn test_make_dummy_connection_manager() {
        let _cm = ConnectionManager::new(
            "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml",
        )
        .expect("Unable to construct new ConnectionManager");
    }
}
