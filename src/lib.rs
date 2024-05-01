pub mod connection;
#[allow(unused_imports)]
pub mod ffi;
pub mod hw;
pub mod node;
pub mod valmem;

#[cfg(test)]
mod test {
    use crate::connection::ConnectionManager;

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

    #[test]
    fn test_getting_node() {
        let mut cm = ConnectionManager::new(
            "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml",
        )
        .expect("Unable to construct new ConnectionManager");

        let mut hw = cm.get_device("dummy.udp").unwrap();
        let value = hw.get_node("REG").unwrap().read().unwrap().value();

        assert!(
            matches!(value, Err(_)),
            "No dispatch, so value should be Err()"
        );
    }

    #[test]
    fn test_bulk_transfer_fail_for_single_register() {
        let mut cm = ConnectionManager::new(
            "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml",
        )
        .expect("Unable to construct new ConnectionManager");

        let mut hw = cm.get_device("dummy.udp").unwrap();
        let value = hw.get_node("REG").unwrap().read_block_dispatch(16);

        assert!(
            matches!(value, Err(_)),
            "No dispatch, so value should be Err()"
        );
    }
}
