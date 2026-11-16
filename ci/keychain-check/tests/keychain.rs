use keychain_check::keychain::{load, remove, store};

#[test]
fn a_key_is_stored_replaced_and_forgotten() {
    let id = "handy-ci-keychain-test";
    remove(id).unwrap();
    assert_eq!(load(id).unwrap(), None);
    store(id, "first").unwrap();
    assert_eq!(load(id).unwrap().as_deref(), Some("first"));
    store(id, "second ก").unwrap();
    assert_eq!(load(id).unwrap().as_deref(), Some("second ก"));
    remove(id).unwrap();
    assert_eq!(load(id).unwrap(), None);
    remove(id).unwrap();
}

/// From inside a tokio runtime, as Handy calls it during a dictation.
#[test]
fn works_from_inside_a_runtime() {
    let rt = tokio_free_runtime();
    let id = "handy-ci-keychain-runtime-test";
    let key = rt(Box::new(move || {
        store(id, "inside").unwrap();
        let key = load(id).unwrap();
        remove(id).unwrap();
        key
    }));
    assert_eq!(key.as_deref(), Some("inside"));
}

/// Runs `f` on a thread that has entered a multi-thread tokio runtime.
fn tokio_free_runtime() -> impl Fn(Box<dyn FnOnce() -> Option<String> + Send>) -> Option<String> {
    |f| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move { tokio::task::spawn(async move { f() }).await.unwrap() })
    }
}
