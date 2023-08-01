use std::{any::Any, collections::HashMap};

use enclose::enclose;
use futures::future;
use serde::Deserialize;
use stremio_derive::Model;

use crate::{
    models::ctx::Ctx,
    runtime::{
        msg::{Action, ActionCtx},
        EnvFutureExt, Runtime, RuntimeAction, TryEnvFuture,
    },
    types::{
        addon::{Descriptor, ResourceResponse},
        library::{LibraryBucket, LibraryItem},
        notifications::{NotificationItem, NotificationsBucket},
        profile::Profile,
        resource::{MetaItemId, VideoId},
        streams::StreamsBucket,
    },
    unit_tests::{default_fetch_handler, Request, TestEnv, FETCH_HANDLER},
};

pub const DATA: &[u8] = include_bytes!("./data.json");

#[derive(Deserialize)]
struct TestData {
    network_requests: HashMap<String, ResourceResponse>,
    addons: Vec<Descriptor>,
    library_items: Vec<LibraryItem>,
    notification_items: Vec<NotificationItem>,
    result: HashMap<MetaItemId, HashMap<VideoId, NotificationItem>>,
}

#[test]
fn notifications() {
    let tests = serde_json::from_slice::<Vec<TestData>>(DATA).unwrap();
    for test in tests {
        #[derive(Model, Clone, Debug)]
        #[model(TestEnv)]
        struct TestModel {
            ctx: Ctx,
        }
        let fetch_handler = enclose!((test.network_requests => network_requests) move |request: Request| -> TryEnvFuture<Box<dyn Any + Send>> {
            if let Some(result) = network_requests.get(&request.url) {
                return future::ok(Box::new(result.to_owned()) as Box<dyn Any + Send>).boxed_env();
            }

            return default_fetch_handler(request);
        });
        let _env_mutex = TestEnv::reset();
        *FETCH_HANDLER.write().unwrap() = Box::new(fetch_handler);
        let (runtime, _rx) = Runtime::<TestEnv, _>::new(
            TestModel {
                ctx: Ctx::new(
                    Profile {
                        addons: test.addons,
                        ..Default::default()
                    },
                    LibraryBucket::new(None, test.library_items),
                    StreamsBucket::default(),
                    NotificationsBucket::new::<TestEnv>(None, test.notification_items),
                ),
            },
            vec![],
            1000,
        );
        TestEnv::run(|| {
            runtime.dispatch(RuntimeAction {
                field: None,
                action: Action::Ctx(ActionCtx::PullNotifications),
            })
        });

        pretty_assertions::assert_eq!(
            runtime.model().unwrap().ctx.notifications.items,
            test.result,
            "Notifications items match"
        );
    }
}