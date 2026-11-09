#![allow(clippy::too_many_lines, clippy::default_trait_access)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

#[allow(unused_imports)]
use std::{str::FromStr, sync::LazyLock};

use app_services::service::service_url::{ServiceSettings, ServiceUrl};

#[allow(dead_code)]
static MATCHES: LazyLock<Vec<(&'static str, ServiceUrl)>> = LazyLock::new(|| {
    vec![
        (
            "apprise://hostname/Token",
            ServiceUrl {
                name: "apprise".to_string(),
                original_url: "apprise://hostname/Token".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ses://user@domain/AccessKeyID/AccessSecretKey/RegionName",
            ServiceUrl {
                name: "ses".to_string(),
                original_url: "ses://user@domain/AccessKeyID/AccessSecretKey/RegionName"
                    .to_string(),
                configuration: "user@domain".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "AccessKeyID".to_string(),
                    "AccessSecretKey".to_string(),
                    "RegionName".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ses://user@domain/AccessKeyID/AccessSecretKey/RegionName/email1/email2/emailN",
            ServiceUrl {
                name: "ses".to_string(),
                original_url:
                    "ses://user@domain/AccessKeyID/AccessSecretKey/RegionName/email1/email2/emailN"
                        .to_string(),
                configuration: "user@domain".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "AccessKeyID".to_string(),
                    "AccessSecretKey".to_string(),
                    "RegionName".to_string(),
                    "email1".to_string(),
                    "email2".to_string(),
                    "emailN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bark://hostname",
            ServiceUrl {
                name: "bark".to_string(),
                original_url: "bark://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bark://hostname/device_key",
            ServiceUrl {
                name: "bark".to_string(),
                original_url: "bark://hostname/device_key".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["device_key".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bark://hostname/device_key1/device_key2/device_keyN",
            ServiceUrl {
                name: "bark".to_string(),
                original_url: "bark://hostname/device_key1/device_key2/device_keyN".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "device_key1".to_string(),
                    "device_key2".to_string(),
                    "device_keyN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "barks://hostname",
            ServiceUrl {
                name: "barks".to_string(),
                original_url: "barks://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "barks://hostname/device_key",
            ServiceUrl {
                name: "barks".to_string(),
                original_url: "barks://hostname/device_key".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["device_key".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "barks://hostname/device_key1/device_key2/device_keyN",
            ServiceUrl {
                name: "barks".to_string(),
                original_url: "barks://hostname/device_key1/device_key2/device_keyN".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "device_key1".to_string(),
                    "device_key2".to_string(),
                    "device_keyN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bluesky://Handle:AppPw",
            ServiceUrl {
                name: "bluesky".to_string(),
                original_url: "bluesky://Handle:AppPw".to_string(),
                configuration: "Handle:AppPw".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bluesky://Handle:AppPw/TargetHandle",
            ServiceUrl {
                name: "bluesky".to_string(),
                original_url: "bluesky://Handle:AppPw/TargetHandle".to_string(),
                configuration: "Handle:AppPw".to_string(),
                parameters: Default::default(),
                arguments: vec!["TargetHandle".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "bluesky://Handle:AppPw/TargetHandle1/TargetHandle2/TargetHandleN",
            ServiceUrl {
                name: "bluesky".to_string(),
                original_url: "bluesky://Handle:AppPw/TargetHandle1/TargetHandle2/TargetHandleN"
                    .to_string(),
                configuration: "Handle:AppPw".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "TargetHandle1".to_string(),
                    "TargetHandle2".to_string(),
                    "TargetHandleN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "chantify://token",
            ServiceUrl {
                name: "chantify".to_string(),
                original_url: "chantify://token".to_string(),
                configuration: "token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "discord://webhook_id/webhook_token",
            ServiceUrl {
                name: "discord".to_string(),
                original_url: "discord://webhook_id/webhook_token".to_string(),
                configuration: "webhook_id".to_string(),
                parameters: Default::default(),
                arguments: vec!["webhook_token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "discord://avatar@webhook_id/webhook_token",
            ServiceUrl {
                name: "discord".to_string(),
                original_url: "discord://avatar@webhook_id/webhook_token".to_string(),
                configuration: "avatar@webhook_id".to_string(),
                parameters: Default::default(),
                arguments: vec!["webhook_token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "emby://user@hostname/",
            ServiceUrl {
                name: "emby".to_string(),
                original_url: "emby://user@hostname/".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "emby://user:password@hostname",
            ServiceUrl {
                name: "emby".to_string(),
                original_url: "emby://user:password@hostname".to_string(),
                configuration: "user:password@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "enigma2://hostname",
            ServiceUrl {
                name: "enigma2".to_string(),
                original_url: "enigma2://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "fcm://project@apikey/DEVICE_ID",
            ServiceUrl {
                name: "fcm".to_string(),
                original_url: "fcm://project@apikey/DEVICE_ID".to_string(),
                configuration: "project@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["DEVICE_ID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "fcm://project@apikey/#TOPIC",
            ServiceUrl {
                name: "fcm".to_string(),
                original_url: "fcm://project@apikey/#TOPIC".to_string(),
                configuration: "project@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "fcm://project@apikey/DEVICE_ID1/#topic1/#topic2/DEVICE_ID2/",
            ServiceUrl {
                name: "fcm".to_string(),
                original_url: "fcm://project@apikey/DEVICE_ID1/#topic1/#topic2/DEVICE_ID2/"
                    .to_string(),
                configuration: "project@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["DEVICE_ID1".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "feishu://token",
            ServiceUrl {
                name: "feishu".to_string(),
                original_url: "feishu://token".to_string(),
                configuration: "token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "flock://token",
            ServiceUrl {
                name: "flock".to_string(),
                original_url: "flock://token".to_string(),
                configuration: "token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ), (
            "flock://botname@token",
            ServiceUrl {
                name: "flock".to_string(),
                original_url: "flock://botname@token".to_string(),
                configuration: "botname@token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "flock://app_token/u:userid",
            ServiceUrl {
                name: "flock".to_string(),
                original_url: "flock://app_token/u:userid".to_string(),
                configuration: "app_token".to_string(),
                parameters: Default::default(),
                arguments: vec!["u:userid".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "flock://app_token/g:channel_id",
            ServiceUrl {
                name: "flock".to_string(),
                original_url: "flock://app_token/g:channel_id".to_string(),
                configuration: "app_token".to_string(),
                parameters: Default::default(),
                arguments: vec!["g:channel_id".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "flock://app_token/u:userid/g:channel_id",
            ServiceUrl {
                name: "flock".to_string(),
                original_url: "flock://app_token/u:userid/g:channel_id".to_string(),
                configuration: "app_token".to_string(),
                parameters: Default::default(),
                arguments: vec!["u:userid".to_string(), "g:channel_id".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "gchat://workspace/key/token",
            ServiceUrl {
                name: "gchat".to_string(),
                original_url: "gchat://workspace/key/token".to_string(),
                configuration: "workspace".to_string(),
                parameters: Default::default(),
                arguments: vec!["key".to_string(), "token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "gotify://hostname/token",
            ServiceUrl {
                name: "gotify".to_string(),
                original_url: "gotify://hostname/token".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "gotifys://hostname/token?priority=high",
            ServiceUrl {
                name: "gotifys".to_string(),
                original_url: "gotifys://hostname/token?priority=high".to_string(),
                configuration: "hostname".to_string(),
                parameters: serde_json::json!({"priority": "high".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "growl://hostname",
            ServiceUrl {
                name: "growl".to_string(),
                original_url: "growl://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "growl://hostname:portno",
            ServiceUrl {
                name: "growl".to_string(),
                original_url: "growl://hostname:portno".to_string(),
                configuration: "hostname:portno".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "growl://password@hostname",
            ServiceUrl {
                name: "growl".to_string(),
                original_url: "growl://password@hostname".to_string(),
                configuration: "password@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "growl://password@hostname:port",
            ServiceUrl {
                name: "growl".to_string(),
                original_url: "growl://password@hostname:port".to_string(),
                configuration: "password@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "growl://hostname?version=1",
            ServiceUrl {
                name: "growl".to_string(),
                original_url: "growl://hostname?version=1".to_string(),
                configuration: "hostname".to_string(),
                parameters: serde_json::json!({"version": "1".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "guilded://webhook_id/webhook_token",
            ServiceUrl {
                name: "guilded".to_string(),
                original_url: "guilded://webhook_id/webhook_token".to_string(),
                configuration: "webhook_id".to_string(),
                parameters: Default::default(),
                arguments: vec!["webhook_token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "guilded://avatar@webhook_id/webhook_token",
            ServiceUrl {
                name: "guilded".to_string(),
                original_url: "guilded://avatar@webhook_id/webhook_token".to_string(),
                configuration: "avatar@webhook_id".to_string(),
                parameters: Default::default(),
                arguments: vec!["webhook_token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "hassio://hostname/accesstoken",
            ServiceUrl {
                name: "hassio".to_string(),
                original_url: "hassio://hostname/accesstoken".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["accesstoken".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "hassio://user@hostname/accesstoken",
            ServiceUrl {
                name: "hassio".to_string(),
                original_url: "hassio://user@hostname/accesstoken".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["accesstoken".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "hassio://user:password@hostname:port/accesstoken",
            ServiceUrl {
                name: "hassio".to_string(),
                original_url: "hassio://user:password@hostname:port/accesstoken".to_string(),
                configuration: "user:password@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["accesstoken".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "hassio://hostname/optional/path/accesstoken",
            ServiceUrl {
                name: "hassio".to_string(),
                original_url: "hassio://hostname/optional/path/accesstoken".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "optional".to_string(),
                    "path".to_string(),
                    "accesstoken".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ifttt://webhooksID/Event",
            ServiceUrl {
                name: "ifttt".to_string(),
                original_url: "ifttt://webhooksID/Event".to_string(),
                configuration: "webhooksID".to_string(),
                parameters: Default::default(),
                arguments: vec!["Event".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ifttt://webhooksID/Event1/Event2/EventN",
            ServiceUrl {
                name: "ifttt".to_string(),
                original_url: "ifttt://webhooksID/Event1/Event2/EventN".to_string(),
                configuration: "webhooksID".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "Event1".to_string(),
                    "Event2".to_string(),
                    "EventN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ifttt://webhooksID/Event1/?+Key=Value",
            ServiceUrl {
                name: "ifttt".to_string(),
                original_url: "ifttt://webhooksID/Event1/?+Key=Value".to_string(),
                configuration: "webhooksID".to_string(),
                parameters: serde_json::json!({" Key": "Value".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["Event1".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ifttt://webhooksID/Event1/?-Key=value1",
            ServiceUrl {
                name: "ifttt".to_string(),
                original_url: "ifttt://webhooksID/Event1/?-Key=value1".to_string(),
                configuration: "webhooksID".to_string(),
                parameters: serde_json::json!({"-Key": "value1".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["Event1".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "join://apikey/device",
            ServiceUrl {
                name: "join".to_string(),
                original_url: "join://apikey/device".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["device".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "join://apikey/device1/device2/deviceN/",
            ServiceUrl {
                name: "join".to_string(),
                original_url: "join://apikey/device1/device2/deviceN/".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "device1".to_string(),
                    "device2".to_string(),
                    "deviceN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "join://apikey/group",
            ServiceUrl {
                name: "join".to_string(),
                original_url: "join://apikey/group".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["group".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "join://apikey/groupA/groupB/groupN",
            ServiceUrl {
                name: "join".to_string(),
                original_url: "join://apikey/groupA/groupB/groupN".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "groupA".to_string(),
                    "groupB".to_string(),
                    "groupN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "join://apikey/DeviceA/groupA/groupN/DeviceN/",
            ServiceUrl {
                name: "join".to_string(),
                original_url: "join://apikey/DeviceA/groupA/groupN/DeviceN/".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "DeviceA".to_string(),
                    "groupA".to_string(),
                    "groupN".to_string(),
                    "DeviceN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "kodi://hostname",
            ServiceUrl {
                name: "kodi".to_string(),
                original_url: "kodi://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "kodi://user@hostname",
            ServiceUrl {
                name: "kodi".to_string(),
                original_url: "kodi://user@hostname".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "kodi://user:password@hostname:port",
            ServiceUrl {
                name: "kodi".to_string(),
                original_url: "kodi://user:password@hostname:port".to_string(),
                configuration: "user:password@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "kumulos://apikey/serverkey",
            ServiceUrl {
                name: "kumulos".to_string(),
                original_url: "kumulos://apikey/serverkey".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["serverkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lametric://apikey@device_ipaddr",
            ServiceUrl {
                name: "lametric".to_string(),
                original_url: "lametric://apikey@device_ipaddr".to_string(),
                configuration: "apikey@device_ipaddr".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lametric://apikey@hostname:port",
            ServiceUrl {
                name: "lametric".to_string(),
                original_url: "lametric://apikey@hostname:port".to_string(),
                configuration: "apikey@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lametric://client_id@client_secret",
            ServiceUrl {
                name: "lametric".to_string(),
                original_url: "lametric://client_id@client_secret".to_string(),
                configuration: "client_id@client_secret".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "line://Token@User",
            ServiceUrl {
                name: "line".to_string(),
                original_url: "line://Token@User".to_string(),
                configuration: "Token@User".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "line://Token/User1/User2/UserN",
            ServiceUrl {
                name: "line".to_string(),
                original_url: "line://Token/User1/User2/UserN".to_string(),
                configuration: "Token".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "User1".to_string(),
                    "User2".to_string(),
                    "UserN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lunasea://user:pass@+FireBaseDevice/",
            ServiceUrl {
                name: "lunasea".to_string(),
                original_url: "lunasea://user:pass@+FireBaseDevice/".to_string(),
                configuration: "user:pass@+FireBaseDevice".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lunasea://user:pass@FireBaseUser/",
            ServiceUrl {
                name: "lunasea".to_string(),
                original_url: "lunasea://user:pass@FireBaseUser/".to_string(),
                configuration: "user:pass@FireBaseUser".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lunasea://user:pass@hostname/+FireBaseDevice/",
            ServiceUrl {
                name: "lunasea".to_string(),
                original_url: "lunasea://user:pass@hostname/+FireBaseDevice/".to_string(),
                configuration: "user:pass@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["+FireBaseDevice".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "lunasea://user:pass@hostname/@FireBaseUser/",
            ServiceUrl {
                name: "lunasea".to_string(),
                original_url: "lunasea://user:pass@hostname/@FireBaseUser/".to_string(),
                configuration: "user:pass@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["@FireBaseUser".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mailgun://user@hostname/apikey",
            ServiceUrl {
                name: "mailgun".to_string(),
                original_url: "mailgun://user@hostname/apikey".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mailgun://user@hostname/apikey/email",
            ServiceUrl {
                name: "mailgun".to_string(),
                original_url: "mailgun://user@hostname/apikey/email".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string(), "email".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mailgun://user@hostname/apikey/email1/email2/emailN",
            ServiceUrl {
                name: "mailgun".to_string(),
                original_url: "mailgun://user@hostname/apikey/email1/email2/emailN".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "apikey".to_string(),
                    "email1".to_string(),
                    "email2".to_string(),
                    "emailN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mailgun://user@hostname/apikey/?name=\"From%20User\"",
            ServiceUrl {
                name: "mailgun".to_string(),
                original_url: "mailgun://user@hostname/apikey/?name=\"From%20User\"".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: serde_json::json!({"name": "\"From User\"".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["apikey".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mastodon://access_key@hostname",
            ServiceUrl {
                name: "mastodon".to_string(),
                original_url: "mastodon://access_key@hostname".to_string(),
                configuration: "access_key@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mastodon://access_key@hostname/@user",
            ServiceUrl {
                name: "mastodon".to_string(),
                original_url: "mastodon://access_key@hostname/@user".to_string(),
                configuration: "access_key@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["@user".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mastodon://access_key@hostname/@user1/@user2/@userN",
            ServiceUrl {
                name: "mastodon".to_string(),
                original_url: "mastodon://access_key@hostname/@user1/@user2/@userN".to_string(),
                configuration: "access_key@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "@user1".to_string(),
                    "@user2".to_string(),
                    "@userN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),(
            "matrix://hostname",
            ServiceUrl {
                name: "matrix".to_string(),
                original_url: "matrix://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrix://user@hostname",
            ServiceUrl {
                name: "matrix".to_string(),
                original_url: "matrix://user@hostname".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrixs://user:pass@hostname:port/#room_alias",
            ServiceUrl {
                name: "matrixs".to_string(),
                original_url: "matrixs://user:pass@hostname:port/#room_alias".to_string(),
                configuration: "user:pass@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrixs://user:pass@hostname:port/!room_id",
            ServiceUrl {
                name: "matrixs".to_string(),
                original_url: "matrixs://user:pass@hostname:port/!room_id".to_string(),
                configuration: "user:pass@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["!room_id".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrixs://user:pass@hostname:port/#room_alias/!room_id/#room2",
            ServiceUrl {
                name: "matrixs".to_string(),
                original_url: "matrixs://user:pass@hostname:port/#room_alias/!room_id/#room2"
                    .to_string(),
                configuration: "user:pass@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrixs://token@hostname:port/?webhook=matrix",
            ServiceUrl {
                name: "matrixs".to_string(),
                original_url: "matrixs://token@hostname:port/?webhook=matrix".to_string(),
                configuration: "token@hostname:port".to_string(),
                parameters: serde_json::json!({"webhook": "matrix".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "matrix://user:token@hostname/?webhook=slack&format=markdown",
            ServiceUrl {
                name: "matrix".to_string(),
                original_url: "matrix://user:token@hostname/?webhook=slack&format=markdown".to_string(),
                configuration: "user:token@hostname".to_string(),
                parameters: serde_json::json!({
                    "webhook": "slack".to_string(),
                    "format": "markdown".to_string(),
                }).as_object().unwrap().clone().into_iter().map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string()))).collect(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmost://hostname/authkey",
            ServiceUrl {
                name: "mmost".to_string(),
                original_url: "mmost://hostname/authkey".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmost://hostname:80/authkey",
            ServiceUrl {
                name: "mmost".to_string(),
                original_url: "mmost://hostname:80/authkey".to_string(),
                configuration: "hostname:80".to_string(),
                parameters: Default::default(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmost://user@hostname:80/authkey",
            ServiceUrl {
                name: "mmost".to_string(),
                original_url: "mmost://user@hostname:80/authkey".to_string(),
                configuration: "user@hostname:80".to_string(),
                parameters: Default::default(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmost://hostname/authkey?channel=channel",
            ServiceUrl {
                name: "mmost".to_string(),
                original_url: "mmost://hostname/authkey?channel=channel".to_string(),
                configuration: "hostname".to_string(),
                parameters: serde_json::json!({"channel": "channel".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmosts://hostname/authkey",
            ServiceUrl {
                name: "mmosts".to_string(),
                original_url: "mmosts://hostname/authkey".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mmosts://user@hostname/authkey",
            ServiceUrl {
                name: "mmosts".to_string(),
                original_url: "mmosts://user@hostname/authkey".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["authkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "workflows://WorkflowID/Signature/",
            ServiceUrl {
                name: "workflows".to_string(),
                original_url: "workflows://WorkflowID/Signature/".to_string(),
                configuration: "WorkflowID".to_string(),
                parameters: Default::default(),
                arguments: vec!["Signature".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "msteams://TokenA/TokenB/TokenC/",
            ServiceUrl {
                name: "msteams".to_string(),
                original_url: "msteams://TokenA/TokenB/TokenC/".to_string(),
                configuration: "TokenA".to_string(),
                parameters: Default::default(),
                arguments: vec!["TokenB".to_string(), "TokenC".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "misskey://access_token@hostname",
            ServiceUrl {
                name: "misskey".to_string(),
                original_url: "misskey://access_token@hostname".to_string(),
                configuration: "access_token@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mqtt://hostname/topic",
            ServiceUrl {
                name: "mqtt".to_string(),
                original_url: "mqtt://hostname/topic".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["topic".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mqtt://user@hostname/topic",
            ServiceUrl {
                name: "mqtt".to_string(),
                original_url: "mqtt://user@hostname/topic".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["topic".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "mqtts://user:pass@hostname:9883/topic",
            ServiceUrl {
                name: "mqtts".to_string(),
                original_url: "mqtts://user:pass@hostname:9883/topic".to_string(),
                configuration: "user:pass@hostname:9883".to_string(),
                parameters: Default::default(),
                arguments: vec!["topic".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ncloud://adminuser:pass@host/User",
            ServiceUrl {
                name: "ncloud".to_string(),
                original_url: "ncloud://adminuser:pass@host/User".to_string(),
                configuration: "adminuser:pass@host".to_string(),
                parameters: Default::default(),
                arguments: vec!["User".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "nclouds://adminuser:pass@host/User1/User2/UserN",
            ServiceUrl {
                name: "nclouds".to_string(),
                original_url: "nclouds://adminuser:pass@host/User1/User2/UserN".to_string(),
                configuration: "adminuser:pass@host".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "User1".to_string(),
                    "User2".to_string(),
                    "UserN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "nctalk://user:pass@host/RoomId",
            ServiceUrl {
                name: "nctalk".to_string(),
                original_url: "nctalk://user:pass@host/RoomId".to_string(),
                configuration: "user:pass@host".to_string(),
                parameters: Default::default(),
                arguments: vec!["RoomId".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "nctalks://user:pass@host/RoomId1/RoomId2/RoomIdN",
            ServiceUrl {
                name: "nctalks".to_string(),
                original_url: "nctalks://user:pass@host/RoomId1/RoomId2/RoomIdN".to_string(),
                configuration: "user:pass@host".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "RoomId1".to_string(),
                    "RoomId2".to_string(),
                    "RoomIdN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "notica://Token/",
            ServiceUrl {
                name: "notica".to_string(),
                original_url: "notica://Token/".to_string(),
                configuration: "Token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "notifiarr://apikey/#channel",
            ServiceUrl {
                name: "notifiarr".to_string(),
                original_url: "notifiarr://apikey/#channel".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "notifiarr://apikey/#channel1/#channel2/#channeln",
            ServiceUrl {
                name: "notifiarr".to_string(),
                original_url: "notifiarr://apikey/#channel1/#channel2/#channeln".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "notifico://ProjectID/MessageHook/",
            ServiceUrl {
                name: "notifico".to_string(),
                original_url: "notifico://ProjectID/MessageHook/".to_string(),
                configuration: "ProjectID".to_string(),
                parameters: Default::default(),
                arguments: vec!["MessageHook".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ntfy://topic/",
            ServiceUrl {
                name: "ntfy".to_string(),
                original_url: "ntfy://topic/".to_string(),
                configuration: "topic".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ntfys://topic/",
            ServiceUrl {
                name: "ntfys".to_string(),
                original_url: "ntfys://topic/".to_string(),
                configuration: "topic".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "o365://TenantID:AccountEmail/ClientID/ClientSecret",
            ServiceUrl {
                name: "o365".to_string(),
                original_url: "o365://TenantID:AccountEmail/ClientID/ClientSecret".to_string(),
                configuration: "TenantID:AccountEmail".to_string(),
                parameters: Default::default(),
                arguments: vec!["ClientID".to_string(), "ClientSecret".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "o365://TenantID:AccountEmail/ClientID/ClientSecret/TargetEmail",
            ServiceUrl {
                name: "o365".to_string(),
                original_url: "o365://TenantID:AccountEmail/ClientID/ClientSecret/TargetEmail"
                    .to_string(),
                configuration: "TenantID:AccountEmail".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ClientID".to_string(),
                    "ClientSecret".to_string(),
                    "TargetEmail".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "o365://TenantID:AccountEmail/ClientID/ClientSecret/TargetEmail1/TargetEmail2/TargetEmailN",
            ServiceUrl {
                name: "o365".to_string(),
                original_url: "o365://TenantID:AccountEmail/ClientID/ClientSecret/TargetEmail1/TargetEmail2/TargetEmailN".to_string(),
                configuration: "TenantID:AccountEmail".to_string(),
                parameters: Default::default(),
                arguments: vec!["ClientID".to_string(), "ClientSecret".to_string(), "TargetEmail1".to_string(), "TargetEmail2".to_string(), "TargetEmailN".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "onesignal://AppID@APIKey/PlayerID",
            ServiceUrl {
                name: "onesignal".to_string(),
                original_url: "onesignal://AppID@APIKey/PlayerID".to_string(),
                configuration: "AppID@APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["PlayerID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "onesignal://TemplateID:AppID@APIKey/UserID",
            ServiceUrl {
                name: "onesignal".to_string(),
                original_url: "onesignal://TemplateID:AppID@APIKey/UserID".to_string(),
                configuration: "TemplateID:AppID@APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["UserID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "onesignal://AppID@APIKey/#IncludeSegment",
            ServiceUrl {
                name: "onesignal".to_string(),
                original_url: "onesignal://AppID@APIKey/#IncludeSegment".to_string(),
                configuration: "AppID@APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "onesignal://AppID@APIKey/Email",
            ServiceUrl {
                name: "onesignal".to_string(),
                original_url: "onesignal://AppID@APIKey/Email".to_string(),
                configuration: "AppID@APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["Email".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "opsgenie://APIKey",
            ServiceUrl {
                name: "opsgenie".to_string(),
                original_url: "opsgenie://APIKey".to_string(),
                configuration: "APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "opsgenie://APIKey/UserID",
            ServiceUrl {
                name: "opsgenie".to_string(),
                original_url: "opsgenie://APIKey/UserID".to_string(),
                configuration: "APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["UserID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "opsgenie://APIKey/#Team",
            ServiceUrl {
                name: "opsgenie".to_string(),
                original_url: "opsgenie://APIKey/#Team".to_string(),
                configuration: "APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "opsgenie://APIKey/*Schedule",
            ServiceUrl {
                name: "opsgenie".to_string(),
                original_url: "opsgenie://APIKey/*Schedule".to_string(),
                configuration: "APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["*Schedule".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "opsgenie://APIKey/^Escalation",
            ServiceUrl {
                name: "opsgenie".to_string(),
                original_url: "opsgenie://APIKey/^Escalation".to_string(),
                configuration: "APIKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["^Escalation".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pagerduty://IntegrationKey@ApiKey",
            ServiceUrl {
                name: "pagerduty".to_string(),
                original_url: "pagerduty://IntegrationKey@ApiKey".to_string(),
                configuration: "IntegrationKey@ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pagerduty://IntegrationKey@ApiKey/Source/Component",
            ServiceUrl {
                name: "pagerduty".to_string(),
                original_url: "pagerduty://IntegrationKey@ApiKey/Source/Component".to_string(),
                configuration: "IntegrationKey@ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["Source".to_string(), "Component".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pagertree://integration_id",
            ServiceUrl {
                name: "pagertree".to_string(),
                original_url: "pagertree://integration_id".to_string(),
                configuration: "integration_id".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "parsep://AppID:MasterKey@Hostname",
            ServiceUrl {
                name: "parsep".to_string(),
                original_url: "parsep://AppID:MasterKey@Hostname".to_string(),
                configuration: "AppID:MasterKey@Hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "parseps://AppID:MasterKey@Hostname",
            ServiceUrl {
                name: "parseps".to_string(),
                original_url: "parseps://AppID:MasterKey@Hostname".to_string(),
                configuration: "AppID:MasterKey@Hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "popcorn://ApiKey/ToPhoneNo",
            ServiceUrl {
                name: "popcorn".to_string(),
                original_url: "popcorn://ApiKey/ToPhoneNo".to_string(),
                configuration: "ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToPhoneNo".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "popcorn://ApiKey/ToPhoneNo1/ToPhoneNo2/ToPhoneNoN/",
            ServiceUrl {
                name: "popcorn".to_string(),
                original_url: "popcorn://ApiKey/ToPhoneNo1/ToPhoneNo2/ToPhoneNoN/".to_string(),
                configuration: "ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ToPhoneNo1".to_string(),
                    "ToPhoneNo2".to_string(),
                    "ToPhoneNoN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "popcorn://ApiKey/ToEmail",
            ServiceUrl {
                name: "popcorn".to_string(),
                original_url: "popcorn://ApiKey/ToEmail".to_string(),
                configuration: "ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToEmail".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "popcorn://ApiKey/ToEmail1/ToEmail2/ToEmailN/",
            ServiceUrl {
                name: "popcorn".to_string(),
                original_url: "popcorn://ApiKey/ToEmail1/ToEmail2/ToEmailN/".to_string(),
                configuration: "ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ToEmail1".to_string(),
                    "ToEmail2".to_string(),
                    "ToEmailN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "popcorn://ApiKey/ToPhoneNo1/ToEmail1/ToPhoneNoN/ToEmailN",
            ServiceUrl {
                name: "popcorn".to_string(),
                original_url: "popcorn://ApiKey/ToPhoneNo1/ToEmail1/ToPhoneNoN/ToEmailN"
                    .to_string(),
                configuration: "ApiKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ToPhoneNo1".to_string(),
                    "ToEmail1".to_string(),
                    "ToPhoneNoN".to_string(),
                    "ToEmailN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "prowl://apikey",
            ServiceUrl {
                name: "prowl".to_string(),
                original_url: "prowl://apikey".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "prowl://apikey/providerkey",
            ServiceUrl {
                name: "prowl".to_string(),
                original_url: "prowl://apikey/providerkey".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["providerkey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pbul://accesstoken",
            ServiceUrl {
                name: "pbul".to_string(),
                original_url: "pbul://accesstoken".to_string(),
                configuration: "accesstoken".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pbul://accesstoken/#channel",
            ServiceUrl {
                name: "pbul".to_string(),
                original_url: "pbul://accesstoken/#channel".to_string(),
                configuration: "accesstoken".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pbul://accesstoken/A_DEVICE_ID",
            ServiceUrl {
                name: "pbul".to_string(),
                original_url: "pbul://accesstoken/A_DEVICE_ID".to_string(),
                configuration: "accesstoken".to_string(),
                parameters: Default::default(),
                arguments: vec!["A_DEVICE_ID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pbul://accesstoken/email@address.com",
            ServiceUrl {
                name: "pbul".to_string(),
                original_url: "pbul://accesstoken/email@address.com".to_string(),
                configuration: "accesstoken".to_string(),
                parameters: Default::default(),
                arguments: vec!["email@address.com".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pbul://accesstoken/#channel/#channel2/email@address.net/DEVICE",
            ServiceUrl {
                name: "pbul".to_string(),
                original_url: "pbul://accesstoken/#channel/#channel2/email@address.net/DEVICE"
                    .to_string(),
                configuration: "accesstoken".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pjet://hostname/secret",
            ServiceUrl {
                name: "pjet".to_string(),
                original_url: "pjet://hostname/secret".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["secret".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pjet://hostname:port/secret",
            ServiceUrl {
                name: "pjet".to_string(),
                original_url: "pjet://hostname:port/secret".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["secret".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pjets://secret@hostname/secret",
            ServiceUrl {
                name: "pjets".to_string(),
                original_url: "pjets://secret@hostname/secret".to_string(),
                configuration: "secret@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["secret".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pjets://hostname:port/secret",
            ServiceUrl {
                name: "pjets".to_string(),
                original_url: "pjets://hostname:port/secret".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["secret".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "push://apikey/",
            ServiceUrl {
                name: "push".to_string(),
                original_url: "push://apikey/".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushed://appkey/appsecret/",
            ServiceUrl {
                name: "pushed".to_string(),
                original_url: "pushed://appkey/appsecret/".to_string(),
                configuration: "appkey".to_string(),
                parameters: Default::default(),
                arguments: vec!["appsecret".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushed://appkey/appsecret/#ChannelAlias",
            ServiceUrl {
                name: "pushed".to_string(),
                original_url: "pushed://appkey/appsecret/#ChannelAlias".to_string(),
                configuration: "appkey".to_string(),
                parameters: Default::default(),
                arguments: vec!["appsecret".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushed://appkey/appsecret/#ChannelAlias1/#ChannelAlias2/#ChannelAliasN",
            ServiceUrl {
                name: "pushed".to_string(),
                original_url:
                    "pushed://appkey/appsecret/#ChannelAlias1/#ChannelAlias2/#ChannelAliasN"
                        .to_string(),
                configuration: "appkey".to_string(),
                parameters: Default::default(),
                arguments: vec!["appsecret".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushed://appkey/appsecret/@UserPushedID",
            ServiceUrl {
                name: "pushed".to_string(),
                original_url: "pushed://appkey/appsecret/@UserPushedID".to_string(),
                configuration: "appkey".to_string(),
                parameters: Default::default(),
                arguments: vec!["appsecret".to_string(), "@UserPushedID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushed://appkey/appsecret/@UserPushedID1/@UserPushedID2/@UserPushedIDN",
            ServiceUrl {
                name: "pushed".to_string(),
                original_url:
                    "pushed://appkey/appsecret/@UserPushedID1/@UserPushedID2/@UserPushedIDN"
                        .to_string(),
                configuration: "appkey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "appsecret".to_string(),
                    "@UserPushedID1".to_string(),
                    "@UserPushedID2".to_string(),
                    "@UserPushedIDN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushme://Token/",
            ServiceUrl {
                name: "pushme".to_string(),
                original_url: "pushme://Token/".to_string(),
                configuration: "Token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pover://user@token",
            ServiceUrl {
                name: "pover".to_string(),
                original_url: "pover://user@token".to_string(),
                configuration: "user@token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pover://user@token/DEVICE",
            ServiceUrl {
                name: "pover".to_string(),
                original_url: "pover://user@token/DEVICE".to_string(),
                configuration: "user@token".to_string(),
                parameters: Default::default(),
                arguments: vec!["DEVICE".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pover://user@token/DEVICE1/DEVICE2/DEVICEN",
            ServiceUrl {
                name: "pover".to_string(),
                original_url: "pover://user@token/DEVICE1/DEVICE2/DEVICEN".to_string(),
                configuration: "user@token".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "DEVICE1".to_string(),
                    "DEVICE2".to_string(),
                    "DEVICEN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "psafer://privatekey",
            ServiceUrl {
                name: "psafer".to_string(),
                original_url: "psafer://privatekey".to_string(),
                configuration: "privatekey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "psafers://privatekey/DEVICE",
            ServiceUrl {
                name: "psafers".to_string(),
                original_url: "psafers://privatekey/DEVICE".to_string(),
                configuration: "privatekey".to_string(),
                parameters: Default::default(),
                arguments: vec!["DEVICE".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "psafer://privatekey/DEVICE1/DEVICE2/DEVICEN",
            ServiceUrl {
                name: "psafer".to_string(),
                original_url: "psafer://privatekey/DEVICE1/DEVICE2/DEVICEN".to_string(),
                configuration: "privatekey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "DEVICE1".to_string(),
                    "DEVICE2".to_string(),
                    "DEVICEN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushy://apikey/DEVICE",
            ServiceUrl {
                name: "pushy".to_string(),
                original_url: "pushy://apikey/DEVICE".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["DEVICE".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushy://apikey/DEVICE1/DEVICE2/DEVICEN",
            ServiceUrl {
                name: "pushy".to_string(),
                original_url: "pushy://apikey/DEVICE1/DEVICE2/DEVICEN".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "DEVICE1".to_string(),
                    "DEVICE2".to_string(),
                    "DEVICEN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushy://apikey/TOPIC",
            ServiceUrl {
                name: "pushy".to_string(),
                original_url: "pushy://apikey/TOPIC".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["TOPIC".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushy://apikey/TOPIC1/TOPIC2/TOPICN",
            ServiceUrl {
                name: "pushy".to_string(),
                original_url: "pushy://apikey/TOPIC1/TOPIC2/TOPICN".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "TOPIC1".to_string(),
                    "TOPIC2".to_string(),
                    "TOPICN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushdeer://pushKey",
            ServiceUrl {
                name: "pushdeer".to_string(),
                original_url: "pushdeer://pushKey".to_string(),
                configuration: "pushKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushdeer://hostname/pushKey",
            ServiceUrl {
                name: "pushdeer".to_string(),
                original_url: "pushdeer://hostname/pushKey".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["pushKey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "pushdeer://hostname:port/pushKey",
            ServiceUrl {
                name: "pushdeer".to_string(),
                original_url: "pushdeer://hostname:port/pushKey".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["pushKey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "reddit://user:password@app_id/app_secret/subreddit",
            ServiceUrl {
                name: "reddit".to_string(),
                original_url: "reddit://user:password@app_id/app_secret/subreddit".to_string(),
                configuration: "user:password@app_id".to_string(),
                parameters: Default::default(),
                arguments: vec!["app_secret".to_string(), "subreddit".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "reddit://user:password@app_id/app_secret/sub1/sub2/subN",
            ServiceUrl {
                name: "reddit".to_string(),
                original_url: "reddit://user:password@app_id/app_secret/sub1/sub2/subN".to_string(),
                configuration: "user:password@app_id".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "app_secret".to_string(),
                    "sub1".to_string(),
                    "sub2".to_string(),
                    "subN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "resend://APIToken:FromEmail/",
            ServiceUrl {
                name: "resend".to_string(),
                original_url: "resend://APIToken:FromEmail/".to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "resend://APIToken:FromEmail/ToEmail",
            ServiceUrl {
                name: "resend".to_string(),
                original_url: "resend://APIToken:FromEmail/ToEmail".to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToEmail".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "resend://APIToken:FromEmail/ToEmail1/ToEmail2/ToEmailN/",
            ServiceUrl {
                name: "resend".to_string(),
                original_url: "resend://APIToken:FromEmail/ToEmail1/ToEmail2/ToEmailN/".to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ToEmail1".to_string(),
                    "ToEmail2".to_string(),
                    "ToEmailN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "revolt://bottoken/ChannelID",
            ServiceUrl {
                name: "revolt".to_string(),
                original_url: "revolt://bottoken/ChannelID".to_string(),
                configuration: "bottoken".to_string(),
                parameters: Default::default(),
                arguments: vec!["ChannelID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "revolt://bottoken/ChannelID1/ChannelID2/ChannelIDN",
            ServiceUrl {
                name: "revolt".to_string(),
                original_url: "revolt://bottoken/ChannelID1/ChannelID2/ChannelIDN".to_string(),
                configuration: "bottoken".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ChannelID1".to_string(),
                    "ChannelID2".to_string(),
                    "ChannelIDN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rocket://user:password@hostname/RoomID/Channel",
            ServiceUrl {
                name: "rocket".to_string(),
                original_url: "rocket://user:password@hostname/RoomID/Channel".to_string(),
                configuration: "user:password@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["RoomID".to_string(), "Channel".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rockets://user:password@hostname:443/#Channel1/#Channel1/RoomID",
            ServiceUrl {
                name: "rockets".to_string(),
                original_url: "rockets://user:password@hostname:443/#Channel1/#Channel1/RoomID"
                    .to_string(),
                configuration: "user:password@hostname:443".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rocket://user:password@hostname/#Channel",
            ServiceUrl {
                name: "rocket".to_string(),
                original_url: "rocket://user:password@hostname/#Channel".to_string(),
                configuration: "user:password@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rocket://webhook@hostname",
            ServiceUrl {
                name: "rocket".to_string(),
                original_url: "rocket://webhook@hostname".to_string(),
                configuration: "webhook@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rockets://webhook@hostname/@User/#Channel",
            ServiceUrl {
                name: "rockets".to_string(),
                original_url: "rockets://webhook@hostname/@User/#Channel".to_string(),
                configuration: "webhook@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["@User".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rsyslog://hostname",
            ServiceUrl {
                name: "rsyslog".to_string(),
                original_url: "rsyslog://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "rsyslog://hostname/Facility",
            ServiceUrl {
                name: "rsyslog".to_string(),
                original_url: "rsyslog://hostname/Facility".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["Facility".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ryver://Organization/Token",
            ServiceUrl {
                name: "ryver".to_string(),
                original_url: "ryver://Organization/Token".to_string(),
                configuration: "Organization".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "ryver://botname@Organization/Token",
            ServiceUrl {
                name: "ryver".to_string(),
                original_url: "ryver://botname@Organization/Token".to_string(),
                configuration: "botname@Organization".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sendgrid://APIToken:FromEmail/",
            ServiceUrl {
                name: "sendgrid".to_string(),
                original_url: "sendgrid://APIToken:FromEmail/".to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sendgrid://APIToken:FromEmail/ToEmail",
            ServiceUrl {
                name: "sendgrid".to_string(),
                original_url: "sendgrid://APIToken:FromEmail/ToEmail".to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToEmail".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sendgrid://APIToken:FromEmail/ToEmail1/ToEmail2/ToEmailN/",
            ServiceUrl {
                name: "sendgrid".to_string(),
                original_url: "sendgrid://APIToken:FromEmail/ToEmail1/ToEmail2/ToEmailN/"
                    .to_string(),
                configuration: "APIToken:FromEmail".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ToEmail1".to_string(),
                    "ToEmail2".to_string(),
                    "ToEmailN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "schan://sendkey/",
            ServiceUrl {
                name: "schan".to_string(),
                original_url: "schan://sendkey/".to_string(),
                configuration: "sendkey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "signal://hostname:port/FromPhoneNo",
            ServiceUrl {
                name: "signal".to_string(),
                original_url: "signal://hostname:port/FromPhoneNo".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["FromPhoneNo".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "signal://hostname:port/FromPhoneNo/ToPhoneNo",
            ServiceUrl {
                name: "signal".to_string(),
                original_url: "signal://hostname:port/FromPhoneNo/ToPhoneNo".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["FromPhoneNo".to_string(), "ToPhoneNo".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "signal://hostname:port/FromPhoneNo/ToPhoneNo1/ToPhoneNo2/ToPhoneNoN/",
            ServiceUrl {
                name: "signal".to_string(),
                original_url:
                    "signal://hostname:port/FromPhoneNo/ToPhoneNo1/ToPhoneNo2/ToPhoneNoN/"
                        .to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "FromPhoneNo".to_string(),
                    "ToPhoneNo1".to_string(),
                    "ToPhoneNo2".to_string(),
                    "ToPhoneNoN".to_string(),
                    String::new(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "spush://apikey",
            ServiceUrl {
                name: "spush".to_string(),
                original_url: "spush://apikey".to_string(),
                configuration: "apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "spush://salt:password@apikey",
            ServiceUrl {
                name: "spush".to_string(),
                original_url: "spush://salt:password@apikey".to_string(),
                configuration: "salt:password@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "spush://apikey?event=Apprise",
            ServiceUrl {
                name: "spush".to_string(),
                original_url: "spush://apikey?event=Apprise".to_string(),
                configuration: "apikey".to_string(),
                parameters: serde_json::json!({"event": "Apprise".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "slack://TokenA/TokenB/TokenC/",
            ServiceUrl {
                name: "slack".to_string(),
                original_url: "slack://TokenA/TokenB/TokenC/".to_string(),
                configuration: "TokenA".to_string(),
                parameters: Default::default(),
                arguments: vec!["TokenB".to_string(), "TokenC".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "slack://TokenA/TokenB/TokenC/Channel",
            ServiceUrl {
                name: "slack".to_string(),
                original_url: "slack://TokenA/TokenB/TokenC/Channel".to_string(),
                configuration: "TokenA".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "TokenB".to_string(),
                    "TokenC".to_string(),
                    "Channel".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "slack://botname@TokenA/TokenB/TokenC/Channel",
            ServiceUrl {
                name: "slack".to_string(),
                original_url: "slack://botname@TokenA/TokenB/TokenC/Channel".to_string(),
                configuration: "botname@TokenA".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "TokenB".to_string(),
                    "TokenC".to_string(),
                    "Channel".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "slack://user@TokenA/TokenB/TokenC/Channel1/Channel2/ChannelN",
            ServiceUrl {
                name: "slack".to_string(),
                original_url: "slack://user@TokenA/TokenB/TokenC/Channel1/Channel2/ChannelN"
                    .to_string(),
                configuration: "user@TokenA".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "TokenB".to_string(),
                    "TokenC".to_string(),
                    "Channel1".to_string(),
                    "Channel2".to_string(),
                    "ChannelN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "smtp2go://user@hostname/apikey",
            ServiceUrl {
                name: "smtp2go".to_string(),
                original_url: "smtp2go://user@hostname/apikey".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "smtp2go://user@hostname/apikey/email",
            ServiceUrl {
                name: "smtp2go".to_string(),
                original_url: "smtp2go://user@hostname/apikey/email".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string(), "email".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "smtp2go://user@hostname/apikey/email1/email2/emailN",
            ServiceUrl {
                name: "smtp2go".to_string(),
                original_url: "smtp2go://user@hostname/apikey/email1/email2/emailN".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "apikey".to_string(),
                    "email1".to_string(),
                    "email2".to_string(),
                    "emailN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "smtp2go://user@hostname/apikey/?name=\"From%20User\"",
            ServiceUrl {
                name: "smtp2go".to_string(),
                original_url: "smtp2go://user@hostname/apikey/?name=\"From%20User\"".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: serde_json::json!({"name": "\"From User\"".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["apikey".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sparkpost://user@hostname/apikey",
            ServiceUrl {
                name: "sparkpost".to_string(),
                original_url: "sparkpost://user@hostname/apikey".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sparkpost://user@hostname/apikey/email",
            ServiceUrl {
                name: "sparkpost".to_string(),
                original_url: "sparkpost://user@hostname/apikey/email".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["apikey".to_string(), "email".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sparkpost://user@hostname/apikey/email1/email2/emailN",
            ServiceUrl {
                name: "sparkpost".to_string(),
                original_url: "sparkpost://user@hostname/apikey/email1/email2/emailN".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "apikey".to_string(),
                    "email1".to_string(),
                    "email2".to_string(),
                    "emailN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "sparkpost://user@hostname/apikey/?name=\"From%20User\"",
            ServiceUrl {
                name: "sparkpost".to_string(),
                original_url: "sparkpost://user@hostname/apikey/?name=\"From%20User\"".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: serde_json::json!({"name": "\"From User\"".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec!["apikey".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "splunk://route_key@apikey",
            ServiceUrl {
                name: "splunk".to_string(),
                original_url: "splunk://route_key@apikey".to_string(),
                configuration: "route_key@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "splunk://route_key@apikey/entity_id",
            ServiceUrl {
                name: "splunk".to_string(),
                original_url: "splunk://route_key@apikey/entity_id".to_string(),
                configuration: "route_key@apikey".to_string(),
                parameters: Default::default(),
                arguments: vec!["entity_id".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "strmlabs://AccessToken/",
            ServiceUrl {
                name: "strmlabs".to_string(),
                original_url: "strmlabs://AccessToken/".to_string(),
                configuration: "AccessToken".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "strmlabs://AccessToken/?name=name&identifier=identifier&amount=0&currency=USD",
            ServiceUrl {
                name: "strmlabs".to_string(),
                original_url: "strmlabs://AccessToken/?name=name&identifier=identifier&amount=0&currency=USD".to_string(),
                configuration: "AccessToken".to_string(),
                parameters: serde_json::json!({
                    "amount": "0".to_string(),
                    "currency": "USD".to_string(),
                    "identifier": "identifier".to_string(),
                    "name": "name".to_string(),
                }).as_object().unwrap().clone().into_iter().map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string()))).collect(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "synology://hostname/token",
            ServiceUrl {
                name: "synology".to_string(),
                original_url: "synology://hostname/token".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec!["token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "synology://hostname:port/token",
            ServiceUrl {
                name: "synology".to_string(),
                original_url: "synology://hostname:port/token".to_string(),
                configuration: "hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec!["token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "syslog://",
            ServiceUrl {
                name: "syslog".to_string(),
                original_url: "syslog://".to_string(),
                configuration: String::new(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "syslog://Facility",
            ServiceUrl {
                name: "syslog".to_string(),
                original_url: "syslog://Facility".to_string(),
                configuration: "Facility".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "tgram://bottoken/ChatID",
            ServiceUrl {
                name: "tgram".to_string(),
                original_url: "tgram://bottoken/ChatID".to_string(),
                configuration: "bottoken".to_string(),
                parameters: Default::default(),
                arguments: vec!["ChatID".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "tgram://bottoken/ChatID1/ChatID2/ChatIDN",
            ServiceUrl {
                name: "tgram".to_string(),
                original_url: "tgram://bottoken/ChatID1/ChatID2/ChatIDN".to_string(),
                configuration: "bottoken".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "ChatID1".to_string(),
                    "ChatID2".to_string(),
                    "ChatIDN".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twitter://CKey/CSecret/AKey/ASecret",
            ServiceUrl {
                name: "twitter".to_string(),
                original_url: "twitter://CKey/CSecret/AKey/ASecret".to_string(),
                configuration: "CKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "CSecret".to_string(),
                    "AKey".to_string(),
                    "ASecret".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twitter://user@CKey/CSecret/AKey/ASecret",
            ServiceUrl {
                name: "twitter".to_string(),
                original_url: "twitter://user@CKey/CSecret/AKey/ASecret".to_string(),
                configuration: "user@CKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "CSecret".to_string(),
                    "AKey".to_string(),
                    "ASecret".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twitter://CKey/CSecret/AKey/ASecret/User1/User2/User2",
            ServiceUrl {
                name: "twitter".to_string(),
                original_url: "twitter://CKey/CSecret/AKey/ASecret/User1/User2/User2"
                    .to_string(),
                configuration: "CKey".to_string(),
                parameters: Default::default(),
                arguments: vec![
                    "CSecret".to_string(),
                    "AKey".to_string(),
                    "ASecret".to_string(),
                    "User1".to_string(),
                    "User2".to_string(),
                    "User2".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twitter://CKey/CSecret/AKey/ASecret?mode=tweet",
            ServiceUrl {
                name: "twitter".to_string(),
                original_url: "twitter://CKey/CSecret/AKey/ASecret?mode=tweet".to_string(),
                configuration: "CKey".to_string(),
                parameters: serde_json::json!({"mode": "tweet".to_string()})
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, either::Left(v.as_str().unwrap().to_string())))
                    .collect(),
                arguments: vec![
                    "CSecret".to_string(),
                    "AKey".to_string(),
                    "ASecret".to_string(),
                ],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twist://pasword:login",
            ServiceUrl {
                name: "twist".to_string(),
                original_url: "twist://pasword:login".to_string(),
                configuration: "pasword:login".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twist://password:login/#channel",
            ServiceUrl {
                name: "twist".to_string(),
                original_url: "twist://password:login/#channel".to_string(),
                configuration: "password:login".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twist://password:login/#team:channel",
            ServiceUrl {
                name: "twist".to_string(),
                original_url: "twist://password:login/#team:channel".to_string(),
                configuration: "password:login".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "twist://password:login/#team:channel1/channel2/#team3:channel",
            ServiceUrl {
                name: "twist".to_string(),
                original_url: "twist://password:login/#team:channel1/channel2/#team3:channel"
                    .to_string(),
                configuration: "password:login".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "wxteams://Token",
            ServiceUrl {
                name: "wxteams".to_string(),
                original_url: "wxteams://Token".to_string(),
                configuration: "Token".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "wecombot://BotKey",
            ServiceUrl {
                name: "wecombot".to_string(),
                original_url: "wecombot://BotKey".to_string(),
                configuration: "BotKey".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "whatsapp://AccessToken@FromPhoneID/ToPhoneNo",
            ServiceUrl {
                name: "whatsapp".to_string(),
                original_url: "whatsapp://AccessToken@FromPhoneID/ToPhoneNo".to_string(),
                configuration: "AccessToken@FromPhoneID".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToPhoneNo".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "whatsapp://Template:AccessToken@FromPhoneID/ToPhoneNo",
            ServiceUrl {
                name: "whatsapp".to_string(),
                original_url: "whatsapp://Template:AccessToken@FromPhoneID/ToPhoneNo"
                    .to_string(),
                configuration: "Template:AccessToken@FromPhoneID".to_string(),
                parameters: Default::default(),
                arguments: vec!["ToPhoneNo".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "wxpusher://AppToken@UserID1/UserID2/UserIDN",
            ServiceUrl {
                name: "wxpusher".to_string(),
                original_url: "wxpusher://AppToken@UserID1/UserID2/UserIDN".to_string(),
                configuration: "AppToken@UserID1".to_string(),
                parameters: Default::default(),
                arguments: vec!["UserID2".to_string(), "UserIDN".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "wxpusher://AppToken@Topic1/Topic2/Topic3",
            ServiceUrl {
                name: "wxpusher".to_string(),
                original_url: "wxpusher://AppToken@Topic1/Topic2/Topic3".to_string(),
                configuration: "AppToken@Topic1".to_string(),
                parameters: Default::default(),
                arguments: vec!["Topic2".to_string(), "Topic3".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "wxpusher://AppToken@UserID1/Topic1/",
            ServiceUrl {
                name: "wxpusher".to_string(),
                original_url: "wxpusher://AppToken@UserID1/Topic1/".to_string(),
                configuration: "AppToken@UserID1".to_string(),
                parameters: Default::default(),
                arguments: vec!["Topic1".to_string(), String::new()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "xbmc://hostname",
            ServiceUrl {
                name: "xbmc".to_string(),
                original_url: "xbmc://hostname".to_string(),
                configuration: "hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "xbmc://user@hostname",
            ServiceUrl {
                name: "xbmc".to_string(),
                original_url: "xbmc://user@hostname".to_string(),
                configuration: "user@hostname".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "xbmc://user:password@hostname:port",
            ServiceUrl {
                name: "xbmc".to_string(),
                original_url: "xbmc://user:password@hostname:port".to_string(),
                configuration: "user:password@hostname:port".to_string(),
                parameters: Default::default(),
                arguments: vec![],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "zulip://botname@Organization/Token",
            ServiceUrl {
                name: "zulip".to_string(),
                original_url: "zulip://botname@Organization/Token".to_string(),
                configuration: "botname@Organization".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "zulip://botname@Organization/Token/Stream",
            ServiceUrl {
                name: "zulip".to_string(),
                original_url: "zulip://botname@Organization/Token/Stream".to_string(),
                configuration: "botname@Organization".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string(), "Stream".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
        (
            "zulip://botname@Organization/Token/Email",
            ServiceUrl {
                name: "zulip".to_string(),
                original_url: "zulip://botname@Organization/Token/Email".to_string(),
                configuration: "botname@Organization".to_string(),
                parameters: Default::default(),
                arguments: vec!["Token".to_string(), "Email".to_string()],
                settings: ServiceSettings::default(),
            },
        ),
    ]
});

#[test]
fn test_from_str_can_parse_example_urls() {
    let urls = MATCHES.iter().map(|(url, _)| url).collect::<Vec<_>>();

    for url in urls {
        let res = ServiceUrl::from_str(url);

        assert!(res.is_ok(), "Failed to parse URL: {}", url);
    }
}

#[test]
fn test_from_str_correctly_parses_urls() {
    for (url, expected) in MATCHES.clone() {
        let res = ServiceUrl::from_str(url);

        assert!(res.is_ok(), "Failed to parse URL: {}", url);

        assert_eq!(res.unwrap(), expected);
    }
}
