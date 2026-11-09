#![allow(clippy::unwrap_used, clippy::expect_used)]

use app_server::config::{ApiServerConfig, DatabaseUrl};

#[test]
fn configuration_debug_output_redacts_secrets_while_serialization_preserves_them() {
    let api = ApiServerConfig {
        host: "127.0.0.1".into(),
        port: 9009,
        jwt_secret: "unique-signing-secret".into(),
    };
    assert!(!format!("{api:?}").contains("unique-signing-secret"));
    assert!(
        serde_json::to_string(&api)
            .unwrap()
            .contains("unique-signing-secret")
    );
    let database = DatabaseUrl::Remote {
        url: "https://alice:unique-db-password@example.com/db?token=another-db-token"
            .parse()
            .unwrap(),
        token: Some("unique-db-token".into()),
    };
    for output in [database.to_string(), format!("{database:?}")] {
        for secret in ["unique-db-password", "another-db-token", "unique-db-token"] {
            assert!(!output.contains(secret), "leaked secret: {output}");
        }
    }
}
