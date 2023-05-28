use rabbitmq_http_client::{blocking::Client, requests::QueueParams};
use serde_json::{Value, Map, json};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_declare_a_quorum_queue() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let vhost = "/";
    let name = "rust.tests.qq.1";

    let _ = rc.delete_queue(&vhost, &name);

    let result1 = rc.get_queue_info(&vhost, &name);
    assert!(!result1.is_ok());

    let mut map = Map::<String, Value>::new();
    map.insert("x-max-length".to_owned(), json!(10_000));
    let optional_args = Some(map);
    let params = QueueParams::new_quorum_queue(&name, optional_args);
    let result2 = rc.declare_queue(&vhost, &params);
    assert!(result2.is_ok());

    let _ = rc.delete_queue(&vhost, &name);
}

#[test]
fn test_declare_a_stream() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let vhost = "/";
    let name = "rust.tests.qq.1";

    let _ = rc.delete_queue(&vhost, &name);

    let result1 = rc.get_queue_info(&vhost, &name);
    assert!(!result1.is_ok());

    let mut map = Map::<String, Value>::new();
    map.insert("x-max-length-bytes".to_owned(), json!(10_000_000));
    let optional_args = Some(map);
    let params = QueueParams::new_stream(&name, optional_args);
    let result2 = rc.declare_queue(&vhost, &params);
    assert!(result2.is_ok());

    let _ = rc.delete_queue(&vhost, &name);
}

#[test]
fn test_delete_queue() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let vhost = "/";
    let name = "rust.tests.cq.10";

    let _ = rc.delete_queue(&vhost, &name);

    let result1 = rc.get_queue_info(&vhost, &name);
    assert!(!result1.is_ok());

    let params = QueueParams::new_exclusive_classic_queue(&name, None);
    let result2 = rc.declare_queue(&vhost, &params);
    assert!(result2.is_ok());

    let _ = rc.delete_queue(&vhost, &name);
    let result3 = rc.get_queue_info(&vhost, &name);
    assert!(!result3.is_ok());
}

#[test]
fn test_list_all_queues() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    
    let result1 = rc.list_queues();
    assert!(result1.is_ok());
}

#[test]
fn test_list_queues_in_a_virtual_host() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));

    let result1 = rc.list_queues_in("/");
    assert!(result1.is_ok());
}