pub mod connection;
#[allow(unused_imports)]
pub mod ffi;
pub mod hw;
pub mod node;
pub mod valmem;

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

    #[test]
    fn test_make_invalid_dunny_connection_manager() {
        assert!(
            matches!(ConnectionManager::new("file://invalid_path_to.xml"), Err(_)),
            "ConnectionManager did not fail"
        );
    }

    #[test]
    fn test_getting_device() {
        let mut cm = ConnectionManager::new(
            "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml",
        )
        .expect("Unable to construct new ConnectionManager");

        cm.get_device("dummy.udp").unwrap();
    }

    #[test]
    fn test_list_devices() {
        let mut cm = ConnectionManager::new(
            "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml",
        )
        .expect("Unable to construct new ConnectionManager");

        assert_ne!(
            cm.list_devices(None).len(),
            0,
            "Devices should have been in connection"
        );
    }
}
