use maa_sys::{task, Assistant, Connection, InstanceOptionKey};

fn create_assistant() -> Assistant {
    let lib_path = std::env::var("MAA_LIB_PATH").expect("MAA_LIB_PATH env not set");
    let resource_path = std::env::var("MAA_RESOURCE_PATH").expect("MAA_RESOURCE_PATH env not set");
    Assistant::registry()
        .with_library(lib_path)
        .with_resource(resource_path)
        .init()
        .expect("Failed to create Assistant")
}

#[test]
fn test_version() {
    let version = create_assistant().version().unwrap();
    assert_ne!(version, "");
}

#[test]
#[ignore = "MaaCore 这个功能好像有点问题，暂时跳过"]
fn test_get_uuid() {
    let assistant = create_assistant();
    let uuid = assistant.get_uuid().unwrap();
    assert_eq!(uuid, "");
}

#[test]
fn test_get_tasks_list() {
    let mut assistant = create_assistant();
    assistant
        .append_task(
            task::StartUpTask::builder()
                .enable(true)
                .client_type("Official")
                .start_game_enabled(true)
                .account_name("123****4567")
                .build()
        )
        .unwrap();

    let tasks = assistant.get_tasks_list().unwrap();
    assert!(!tasks.is_empty());
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_type(), "StartUp");
}

#[test]
#[ignore = "不太好测试，单独跑能跑通就行"]
fn test_connect_device() {
    let lib_path = std::env::var("MAA_LIB_PATH").expect("MAA_LIB_PATH env not set");
    let resource_path = std::env::var("MAA_RESOURCE_PATH").expect("MAA_RESOURCE_PATH env not set");
    let mut assistant = Assistant::registry()
        .with_library(lib_path)
        .with_resource(resource_path)
        .with_callback(|msg_id, details| {
            let details_json = details.as_object().unwrap();
            println!("收到回调: msg_id={:?}\n details={:?}", msg_id, details_json);
        })
        .init()
        .unwrap();

    assistant
        .set_instance_option(InstanceOptionKey::TouchMode, "adb")
        .unwrap();
    assistant
        .connect(Connection::adb("adb", "192.168.20.29:44847"), None)
        .unwrap();

    if !assistant.is_connected() {
        println!("connect failed");
        return;
    }

    assistant
        .append_task(
            task::StartUpTask::builder()
                .enable(true)
                .client_type("Official")
                .start_game_enabled(true)
                .build()
        )
        .unwrap();

    assistant.start().unwrap();

    println!("should be running");
    std::thread::sleep(std::time::Duration::from_secs(60)); // 等待60秒

    assistant.stop().unwrap();
}
