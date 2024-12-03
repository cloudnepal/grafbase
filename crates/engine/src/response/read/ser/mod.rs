mod data;
mod errors;

use serde::ser::SerializeMap;

use crate::response::{ExecutedResponse, RefusedRequestResponse, RequestErrorResponse, Response, ResponseKeys};

impl<OnOperationResponseHookOutput> serde::Serialize for Response<OnOperationResponseHookOutput> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Response::Executed(ExecutedResponse {
                schema,
                operation,
                data,
                errors,
                extensions,
                ..
            }) => {
                let mut map = serializer.serialize_map(None)?;

                let keys = &operation.solved.response_keys;
                if let Some(data) = data {
                    map.serialize_entry(
                        "data",
                        &data::SerializableResponseData {
                            ctx: data::Context { keys, data, schema },
                        },
                    )?;
                } else {
                    map.serialize_entry("data", &())?;
                }

                if !errors.is_empty() {
                    map.serialize_entry("errors", &errors::SerializableErrors { keys, errors })?;
                }

                if let Some(ext) = extensions.as_ref().filter(|ext| !ext.is_emtpy()) {
                    map.serialize_entry("extensions", ext)?;
                }

                map.end()
            }
            Response::RequestError(RequestErrorResponse { errors, extensions, .. }) => {
                let mut map = serializer.serialize_map(None)?;
                // Shouldn't happen, but better safe than sorry.
                if !errors.is_empty() {
                    let empty_keys = ResponseKeys::default();
                    map.serialize_entry(
                        "errors",
                        &errors::SerializableErrors {
                            keys: &empty_keys,
                            errors,
                        },
                    )?;
                }
                if let Some(ext) = extensions.as_ref().filter(|ext| !ext.is_emtpy()) {
                    map.serialize_entry("extensions", ext)?;
                }

                map.end()
            }
            Response::RefusedRequest(RefusedRequestResponse { errors, extensions, .. }) => {
                let mut map = serializer.serialize_map(None)?;

                // There shouldn't be any response paths needing this, but better safe than sorry.
                let empty_keys = ResponseKeys::default();
                map.serialize_entry(
                    "errors",
                    &errors::SerializableErrors {
                        keys: &empty_keys,
                        errors,
                    },
                )?;

                if let Some(ext) = extensions.as_ref().filter(|ext| !ext.is_emtpy()) {
                    map.serialize_entry("extensions", ext)?;
                }

                map.end()
            }
        }
    }
}