use http::HeaderMap;
use runtime::{
    error::GraphqlError,
    hooks::{DynHookContext, DynHooks, EdgeDefinition},
};

use super::with_engine_for_auth;

#[test]
fn arguments_are_provided() {
    struct TestHooks;

    #[async_trait::async_trait]
    impl DynHooks for TestHooks {
        async fn authorize_edge_pre_execution(
            &self,
            _context: &DynHookContext,
            _definition: EdgeDefinition<'_>,
            arguments: serde_json::Value,
            _metadata: Option<serde_json::Value>,
        ) -> Result<(), GraphqlError> {
            #[derive(serde::Deserialize)]
            struct Arguments {
                id: i64,
            }
            let Arguments { id } = serde_json::from_value(arguments).unwrap();
            if id < 100 {
                Err(format!("Unauthorized ID: {id}").into())
            } else {
                Ok(())
            }
        }
    }

    with_engine_for_auth(TestHooks, |engine| async move {
        let response = engine
            .execute("query { check { authorizedWithId(id: 791) } }")
            .by_client("hi", "")
            .await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": {
            "check": {
              "authorizedWithId": "You have access to the sensistive data"
            }
          }
        }
        "###);

        let response = engine.execute("query { check { authorizedWithId(id: 0) } }").await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": null,
          "errors": [
            {
              "message": "Unauthorized ID: 0",
              "path": [
                "check",
                "authorizedWithId"
              ]
            }
          ]
        }
        "###);
    });
}

#[test]
fn metadata_is_provided() {
    struct TestHooks;

    const NULL: serde_json::Value = serde_json::Value::Null;

    fn extract_role(metadata: Option<&serde_json::Value>) -> Option<&str> {
        metadata
            .unwrap_or(&NULL)
            .as_array()?
            .first()?
            .as_array()?
            .first()?
            .as_str()
    }

    #[async_trait::async_trait]
    impl DynHooks for TestHooks {
        async fn authorize_edge_pre_execution(
            &self,
            _context: &DynHookContext,
            _definition: EdgeDefinition<'_>,
            _arguments: serde_json::Value,
            metadata: Option<serde_json::Value>,
        ) -> Result<(), GraphqlError> {
            if extract_role(metadata.as_ref()) == Some("admin") {
                Ok(())
            } else {
                Err("Unauthorized role".into())
            }
        }
    }

    with_engine_for_auth(TestHooks, |engine| async move {
        let response = engine
            .execute(
                r#"
                query {
                    ok: nullableCheck {
                        authorizedWithMetadata
                    }
                    noMetadata: nullableCheck {
                        authorized
                    }
                }
                "#,
            )
            .await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": {
            "ok": {
              "authorizedWithMetadata": "You have access"
            },
            "noMetadata": null
          },
          "errors": [
            {
              "message": "Unauthorized role",
              "path": [
                "noMetadata",
                "authorized"
              ]
            }
          ]
        }
        "###);
    });
}

#[test]
fn definition_is_provided() {
    struct TestHooks;

    #[async_trait::async_trait]
    impl DynHooks for TestHooks {
        async fn authorize_edge_pre_execution(
            &self,
            _context: &DynHookContext,
            definition: EdgeDefinition<'_>,
            _arguments: serde_json::Value,
            _metadata: Option<serde_json::Value>,
        ) -> Result<(), GraphqlError> {
            if definition.parent_type_name == "Check" && definition.field_name == "authorized" {
                Ok(())
            } else {
                Err("Wrong definition".into())
            }
        }
    }

    with_engine_for_auth(TestHooks, |engine| async move {
        let response = engine
            .execute(
                r#"
                query {
                    ok: nullableCheck {
                        authorized
                    }
                    wrongField: nullableCheck {
                        authorizedWithMetadata
                    }
                    wrongType: nullableOtherCheck {
                        authorized
                    }
                }
                "#,
            )
            .await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": {
            "ok": {
              "authorized": "You have access"
            },
            "wrongField": null,
            "wrongType": null
          },
          "errors": [
            {
              "message": "Wrong definition",
              "path": [
                "wrongField",
                "authorizedWithMetadata"
              ]
            },
            {
              "message": "Wrong definition",
              "path": [
                "wrongType",
                "authorized"
              ]
            }
          ]
        }
        "###);
    });
}

#[test]
fn context_is_propagated() {
    struct TestHooks;

    #[async_trait::async_trait]
    impl DynHooks for TestHooks {
        async fn on_gateway_request(
            &self,
            context: &mut DynHookContext,
            headers: HeaderMap,
        ) -> Result<HeaderMap, GraphqlError> {
            if let Some(client) = headers
                .get("x-grafbase-client-name")
                .and_then(|value| value.to_str().ok())
            {
                context.insert("client", client);
            }
            Ok(headers)
        }

        async fn authorize_edge_pre_execution(
            &self,
            context: &DynHookContext,
            _definition: EdgeDefinition<'_>,
            _arguments: serde_json::Value,
            _metadata: Option<serde_json::Value>,
        ) -> Result<(), GraphqlError> {
            if context.get("client").is_some() {
                Ok(())
            } else {
                Err("Missing client".into())
            }
        }
    }

    with_engine_for_auth(TestHooks, |engine| async move {
        let response = engine
            .execute("query { check { authorized } }")
            .by_client("hi", "")
            .await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": {
            "check": {
              "authorized": "You have access"
            }
          }
        }
        "###);

        let response = engine.execute("query { check { authorized } }").await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": null,
          "errors": [
            {
              "message": "Missing client",
              "path": [
                "check",
                "authorized"
              ]
            }
          ]
        }
        "###);
    });
}

#[test]
fn error_propagation() {
    struct TestHooks;

    #[async_trait::async_trait]
    impl DynHooks for TestHooks {
        async fn authorize_edge_pre_execution(
            &self,
            _context: &DynHookContext,
            _definition: EdgeDefinition<'_>,
            _arguments: serde_json::Value,
            _metadata: Option<serde_json::Value>,
        ) -> Result<(), GraphqlError> {
            Err("Broken".into())
        }
    }

    with_engine_for_auth(TestHooks, |engine| async move {
        let response = engine
            .execute(
                r#"
                query {
                    check {
                        authorized
                    }
                }
                "#,
            )
            .await;
        insta::assert_json_snapshot!(response, @r###"
        {
          "data": null,
          "errors": [
            {
              "message": "Broken",
              "path": [
                "check",
                "authorized"
              ]
            }
          ]
        }
        "###);
    });
}