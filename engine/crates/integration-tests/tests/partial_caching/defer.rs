use integration_tests::{
    runtime,
    udfs::{RustResolver, RustUdfs},
    EngineBuilder,
};
use runtime::udf::UdfResponse;
use serde_json::json;

const SCHEMA: &str = r#"
    extend schema @experimental(partialCaching: true)

    type Query {
        user: User @resolver(name: "user")
    }

    type User {
        name: String @cache(maxAge: 140)
        email: String @cache(maxAge: 130)
        someConstant: String @cache(maxAge: 120)
        uncached: String
    }
"#;

#[test]
fn smoke_test() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(SCHEMA)
            .with_custom_resolvers(RustUdfs::new().resolver("user", UserResolver::default()))
            .gateway_builder()
            .await
            .build();

        const QUERY: &str = r#"
            query {
                user {
                    name
                    ... @defer(label: "woo") {
                        email
                        someConstant
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "label": "woo",
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);

        // Call it again and see what has been cached/not
        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1",
                "email": "1@example.com",
                "someConstant": "blah 1"
              }
            },
            "hasNext": false
          }
        ]
        "###);
    });
}

#[test]
fn test_defer_with_uncached_field() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(SCHEMA)
            .with_custom_resolvers(RustUdfs::new().resolver("user", UserResolver::default()))
            .gateway_builder()
            .await
            .build();

        const QUERY: &str = r#"
            query {
                user {
                    name
                    ... @defer(label: "woo") {
                        email
                        someConstant
                        uncached
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "label": "woo",
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1",
              "uncached": "dont cache me bro 1"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);

        // Call it again and see what has been cached/not
        // I expect all the fields except uncached to still have 1 in their contents
        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "label": "woo",
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1",
              "uncached": "dont cache me bro 2"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);
    });
}

#[test]
fn test_unnamed_defer() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(SCHEMA)
            .with_custom_resolvers(RustUdfs::new().resolver("user", UserResolver::default()))
            .gateway_builder()
            .await
            .build();

        const QUERY: &str = r#"
            query {
                user {
                    name
                    ... @defer {
                        email
                        someConstant
                        uncached
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1",
              "uncached": "dont cache me bro 1"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);
    });
}

#[test]
fn test_multiple_unnamed_defers() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(SCHEMA)
            .with_custom_resolvers(RustUdfs::new().resolver("user", UserResolver::default()))
            .gateway_builder()
            .await
            .build();

        const QUERY: &str = r#"
            query {
                user {
                    name
                    ... @defer {
                        email
                    }
                    ... @defer {
                        someConstant
                    }
                    ... @defer {
                        uncached
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        // Note: technically this is wrong - since each defer chunk ends up containing
        // the fields of all the defers that happened before.  But I'll revisit that
        // in GB-6982
        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com"
            },
            "path": [
              "user"
            ],
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1"
            },
            "path": [
              "user"
            ],
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1",
              "uncached": "dont cache me bro 1"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);
    });
}

#[test]
fn test_nested_defers() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(SCHEMA)
            .with_custom_resolvers(RustUdfs::new().resolver("user", UserResolver::default()))
            .gateway_builder()
            .await
            .build();

        const QUERY: &str = r#"
            query {
                user {
                    name
                    ... @defer {
                        email
                        ... @defer {
                            someConstant
                            ... @defer {
                                uncached
                            }
                        }
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        // Note: technically this is wrong - since each defer chunk ends up containing
        // the fields of all the defers that happened before.  But I'll revisit that
        // in GB-6982
        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "user": {
                "name": "Jo 1"
              }
            },
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com"
            },
            "path": [
              "user"
            ],
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1"
            },
            "path": [
              "user"
            ],
            "hasNext": true
          },
          {
            "data": {
              "name": "Jo 1",
              "email": "1@example.com",
              "someConstant": "blah 1",
              "uncached": "dont cache me bro 1"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);
    });
}

#[test]
fn test_when_defer_served_entirely_from_cache() {
    runtime().block_on(async {
        let gateway = EngineBuilder::new(
            r#"
                extend schema @experimental(partialCaching: true)

                type Query {
                   times: Times @resolver(name: "nope")
                   user(id: ID!): User @resolver(name: "nope")
                }

                type Times {
                   now: String @resolver(name: "time")
                }

                type User {
                    reallyExpensiveField: String
                        @resolver(name: "reallyExpensiveField")
                        @cache(maxAge: 120)
                }
            "#,
        )
        .with_custom_resolvers(
            RustUdfs::new()
                .resolver("nope", json!({}))
                .resolver("reallyExpensiveField", json!("hello"))
                .resolver("time", json!("i'm not really the time")),
        )
        .gateway_builder()
        .await
        .build();

        const QUERY: &str = r#"
            query Times {
                times {
                    now
                }
                user(id: "1") {
                    ... @defer(label: "hello") {
                        reallyExpensiveField
                    }
                }
            }
        "#;

        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "times": {
                "now": "i'm not really the time"
              },
              "user": {}
            },
            "hasNext": true
          },
          {
            "label": "hello",
            "data": {
              "reallyExpensiveField": "hello"
            },
            "path": [
              "user"
            ],
            "hasNext": false
          }
        ]
        "###);

        let responses = gateway.execute(QUERY).collect().await;

        insta::assert_json_snapshot!(responses, @r###"
        [
          {
            "data": {
              "times": {
                "now": "i'm not really the time"
              },
              "user": {
                "reallyExpensiveField": "hello"
              }
            },
            "hasNext": false
          }
        ]
        "###);
    });
}

#[derive(Default)]
pub struct UserResolver {
    call_count: usize,
}

impl RustResolver for UserResolver {
    fn invoke(
        &mut self,
        _payload: runtime::udf::CustomResolverRequestPayload,
    ) -> Result<UdfResponse, runtime::udf::UdfError> {
        self.call_count += 1;
        let call_count = self.call_count;

        let name = format!("Jo {call_count}");
        let email = format!("{call_count}@example.com");
        let constant = format!("blah {call_count}");
        let uncached = format!("dont cache me bro {call_count}");

        Ok(UdfResponse::Success(json!({
            "name": name,
            "email": email,
            "someConstant": constant,
            "uncached": uncached
        })))
    }
}
