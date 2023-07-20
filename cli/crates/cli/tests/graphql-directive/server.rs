use std::net::SocketAddr;

use async_graphql::{EmptyMutation, EmptySubscription, Interface, Object, Schema, SimpleObject, Union, ID};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{http::HeaderMap, routing::post, Router};

async fn graphql_handler(headers: HeaderMap, req: GraphQLRequest) -> GraphQLResponse {
    let headers = headers
        .into_iter()
        .map(|(name, value)| {
            (
                name.map(|name| name.to_string()).unwrap_or_default(),
                String::from_utf8_lossy(value.as_bytes()).to_string(),
            )
        })
        .collect();
    let schema = Schema::build(Query { headers }, EmptyMutation, EmptySubscription).finish();

    schema.execute(req.into_inner()).await.into()
}

pub(crate) async fn run(port: u16) {
    let app = Router::new().route("/", post(graphql_handler));

    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    });
}

struct Query {
    headers: Vec<(String, String)>,
}

#[Object]
impl Query {
    async fn pull_request_or_issue(&self, id: ID) -> Option<PullRequestOrIssue> {
        if id == "1" {
            return Some(PullRequestOrIssue::PullRequest(PullRequest {
                title: "Creating the thing".into(),
                checks: vec!["Success!".into()],
                author: UserOrBot::User(User {
                    name: "Jim".into(),
                    email: "jim@example.com".into(),
                }),
            }));
        } else if id == "2" {
            return Some(PullRequestOrIssue::PullRequest(PullRequest {
                title: "Some bot PR".into(),
                checks: vec!["Success!".into()],
                author: UserOrBot::Bot(Bot { id: "123".into() }),
            }));
        } else if id == "3" {
            return Some(PullRequestOrIssue::Issue(Issue {
                title: "Everythings fucked".into(),
                author: UserOrBot::User(User {
                    name: "The Pessimist".into(),
                    email: "pessimist@example.com".into(),
                }),
            }));
        }
        None
    }

    async fn headers(&self) -> Vec<Header> {
        self.headers
            .clone()
            .into_iter()
            .map(|(name, value)| Header { name, value })
            .collect()
    }
}

#[derive(SimpleObject)]
struct Header {
    name: String,
    value: String,
}

#[derive(SimpleObject)]
struct PullRequest {
    title: String,
    checks: Vec<String>,
    author: UserOrBot,
}

#[derive(SimpleObject)]
struct Issue {
    title: String,
    author: UserOrBot,
}

#[derive(Interface)]
#[graphql(field(name = "title", type = "String"), field(name = "author", type = "UserOrBot"))]
enum PullRequestOrIssue {
    PullRequest(PullRequest),
    Issue(Issue),
}

#[derive(Union, Clone)]
enum UserOrBot {
    User(User),
    Bot(Bot),
}

#[derive(SimpleObject, Clone)]
struct User {
    name: String,
    email: String,
}

#[derive(SimpleObject, Clone)]
struct Bot {
    id: ID,
}

impl From<&UserOrBot> for UserOrBot {
    fn from(value: &UserOrBot) -> Self {
        value.clone()
    }
}