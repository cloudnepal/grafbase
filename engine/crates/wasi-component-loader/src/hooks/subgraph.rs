use url::Url;

use crate::{
    context::SharedContextMap,
    names::{ON_SUBGRAGH_REQUEST_HOOK_FUNCTION, SUBGRAPH_REQUEST_INTERFACE},
    ComponentLoader, GuestResult,
};

use super::{component_instance, ComponentInstance};

component_instance!(SubgraphComponentInstance: SUBGRAPH_REQUEST_INTERFACE);

impl SubgraphComponentInstance {
    /// Called just before sending a HTTP request to a subgraph
    pub async fn on_subgraph_request(
        &mut self,
        context: SharedContextMap,
        subgraph_name: &str,
        method: http::Method,
        url: &Url,
        headers: http::HeaderMap,
    ) -> crate::Result<http::HeaderMap> {
        let Some(hook) = self.get_hook::<_, (GuestResult<()>,)>(ON_SUBGRAGH_REQUEST_HOOK_FUNCTION) else {
            return Ok(headers);
        };

        let subgraph_name = subgraph_name.to_string();
        let url = url.to_string();
        let method = method.to_string();
        // adds the data to the shared memory
        let context = self.store.data_mut().push_resource(context)?;
        let headers = self.store.data_mut().push_resource(headers)?;

        // we need to take the pointers now, because a resource is not Copy and we need
        // the pointers to get the data back from the shared memory.
        let headers_rep = headers.rep();
        let context_rep = context.rep();

        let result = hook
            .call_async(&mut self.store, (context, subgraph_name, method, url, headers))
            .await;

        if result.is_err() {
            self.poisoned = true;
        } else {
            hook.post_return_async(&mut self.store).await?;
        }

        result?.0?;

        // take the data back from the shared memory
        self.store.data_mut().take_resource::<SharedContextMap>(context_rep)?;
        let headers = self.store.data_mut().take_resource(headers_rep)?;

        Ok(headers)
    }
}
