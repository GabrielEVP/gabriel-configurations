use gabriel::{
    config::DesiredFile,
    state::{Decision, Store, decide},
};
use std::fs;

#[test]
fn three_way_decisions_cover_edits_and_removals() {
    let base = Some(b"base".as_slice());
    let local = Some(b"local".as_slice());
    let incoming = Some(b"incoming".as_slice());
    assert_eq!(decide(base, base, incoming), Decision::Incoming);
    assert_eq!(decide(base, local, base), Decision::Local);
    assert_eq!(decide(base, incoming, incoming), Decision::Incoming);
    assert_eq!(decide(base, local, incoming), Decision::Conflict);
    assert_eq!(decide(base, base, None), Decision::Incoming);
    assert_eq!(decide(base, local, None), Decision::Conflict);
    assert_eq!(decide(base, None, base), Decision::Local);
}

#[test]
fn backs_up_initial_files_preserves_unmanaged_and_is_idempotent() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("config");
    fs::write(&destination, "previous").unwrap();
    fs::write(root.path().join("unmanaged"), "untouched").unwrap();
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let files = [DesiredFile {
        path: destination.clone(),
        content: b"managed".to_vec(),
        local_only: false,
    }];
    let transaction = store.apply("test", &files).unwrap().unwrap();
    assert_eq!(fs::read(&destination).unwrap(), b"managed");
    assert!(store.apply("test", &files).unwrap().is_none());
    assert_eq!(
        fs::read(root.path().join("unmanaged")).unwrap(),
        b"untouched"
    );
    store.restore(&transaction).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"previous");
}

#[test]
fn preserves_local_edits_and_blocks_divergent_module_atomically() {
    let root = tempfile::tempdir().unwrap();
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let mut files = vec![
        DesiredFile {
            path: root.path().join("one"),
            content: b"base".to_vec(),
            local_only: false,
        },
        DesiredFile {
            path: root.path().join("two"),
            content: b"base".to_vec(),
            local_only: false,
        },
    ];
    store.apply("test", &files).unwrap();
    fs::write(&files[0].path, "local").unwrap();
    assert!(store.apply("test", &files).unwrap().is_none());
    files[0].content = b"incoming".to_vec();
    files[1].content = b"incoming".to_vec();
    assert!(
        store
            .apply("test", &files)
            .unwrap_err()
            .contains("conflict")
    );
    assert_eq!(fs::read(&files[0].path).unwrap(), b"local");
    assert_eq!(fs::read(&files[1].path).unwrap(), b"base");
}

#[test]
fn restores_broken_symlink_without_following_it() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("zshrc");
    std::os::unix::fs::symlink("/missing/old-clone", &destination).unwrap();
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let transaction = store
        .apply(
            "zsh",
            &[DesiredFile {
                path: destination.clone(),
                content: b"new".to_vec(),
                local_only: false,
            }],
        )
        .unwrap()
        .unwrap();
    store.restore(&transaction).unwrap();
    assert_eq!(
        fs::read_link(destination).unwrap().to_str().unwrap(),
        "/missing/old-clone"
    );
}

#[test]
fn locks_store_and_rejects_unknown_state_schema() {
    let root = tempfile::tempdir().unwrap();
    let store = Store::open(root.path()).unwrap();
    assert!(Store::open(root.path()).is_err());
    drop(store);
    fs::write(
        root.path().join("state.json"),
        r#"{"schema_version":99,"modules":{}}"#,
    )
    .unwrap();
    assert!(Store::open(root.path()).is_err());
}

#[test]
fn refuses_to_overwrite_credentials_and_later_changes_during_restore() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("credentials");
    fs::write(&destination, "private").unwrap();
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let files = [DesiredFile {
        path: destination.clone(),
        content: b"template".to_vec(),
        local_only: true,
    }];
    assert!(store.apply("secret", &files).unwrap().is_none());
    assert_eq!(fs::read(&destination).unwrap(), b"private");
    let transaction = store
        .apply(
            "other",
            &[DesiredFile {
                path: root.path().join("other"),
                content: b"first".to_vec(),
                local_only: false,
            }],
        )
        .unwrap()
        .unwrap();
    fs::write(root.path().join("other"), "local").unwrap();
    assert!(store.restore(&transaction).is_err());
    assert_eq!(fs::read(root.path().join("other")).unwrap(), b"local");
}

#[test]
fn adopts_identical_existing_file_for_future_conflict_detection() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("config");
    fs::write(&destination, "base").unwrap();
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let mut files = [DesiredFile {
        path: destination.clone(),
        content: b"base".to_vec(),
        local_only: false,
    }];
    store.apply("test", &files).unwrap();
    fs::write(&destination, "local").unwrap();
    files[0].content = b"incoming".to_vec();
    assert!(store.apply("test", &files).is_err());
    assert_eq!(fs::read(destination).unwrap(), b"local");
}

#[test]
fn resolves_conflict_with_local_or_incoming_choice() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("config");
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let files = [DesiredFile {
        path: destination.clone(),
        content: b"base".to_vec(),
        local_only: false,
    }];
    store.apply("test", &files).unwrap();
    fs::write(&destination, "local").unwrap();
    store
        .resolve("test", &destination, b"incoming", false)
        .unwrap();
    assert_eq!(fs::read(&destination).unwrap(), b"local");
    store.resolve("test", &destination, b"next", true).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"next");
}

#[test]
fn automatic_policy_replaces_selected_conflicts_atomically() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("config");
    let mut store = Store::open(&root.path().join("state")).unwrap();
    let files = [DesiredFile {
        path: destination.clone(),
        content: b"base".to_vec(),
        local_only: false,
    }];
    store.apply("test", &files).unwrap();
    fs::write(&destination, "local").unwrap();
    let incoming = [DesiredFile {
        path: destination.clone(),
        content: b"incoming".to_vec(),
        local_only: false,
    }];
    let transaction = store
        .apply_with_policy("test", &incoming, true)
        .unwrap()
        .unwrap();
    assert_eq!(fs::read(&destination).unwrap(), b"incoming");
    store.restore(&transaction).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"local");
}

#[test]
fn recovers_uncommitted_writes_before_accepting_another_operation() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("config");
    fs::write(&destination, "before").unwrap();
    let state_root = root.path().join("state");
    let mut store = Store::open(&state_root).unwrap();
    let transaction = store
        .apply(
            "test",
            &[DesiredFile {
                path: destination.clone(),
                content: b"after".to_vec(),
                local_only: false,
            }],
        )
        .unwrap()
        .unwrap();
    drop(store);
    let journal = state_root
        .join("transactions")
        .join(format!("{transaction}.json"));
    let mut document: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal).unwrap()).unwrap();
    document["status"] = "pending".into();
    fs::write(journal, serde_json::to_vec(&document).unwrap()).unwrap();
    let _store = Store::open(&state_root).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"before");
}
