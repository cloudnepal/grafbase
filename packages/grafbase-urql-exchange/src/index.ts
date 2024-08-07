import { isLiveQueryOperationDefinitionNode } from '@n1ru4l/graphql-live-query'
import { applyLiveQueryJSONPatch } from '@n1ru4l/graphql-live-query-patch-json-patch'
import {
  applyAsyncIterableIteratorToSink,
  makeAsyncIterableIteratorFromSink
} from '@n1ru4l/push-pull-async-iterable-iterator'
import { Repeater } from '@repeaterjs/repeater'
import {
  Exchange,
  ExecutionResult,
  Operation,
  subscriptionExchange
} from '@urql/core'
import { DocumentNode } from 'graphql'
import ReconnectingEventSource from 'reconnecting-eventsource'
import { filter, merge, pipe, share } from 'wonka'

const makeEventStreamSource = (url: string) => {
  const eventSource = new ReconnectingEventSource(url)
  return applyLiveQueryJSONPatch(
    new Repeater<ExecutionResult>(async (push, end) => {
      eventSource.onmessage = (event) => {
        const data = JSON.parse(event.data)
        push(data)
        if (eventSource.readyState === 2) {
          end()
        }
      }
      eventSource.onerror = (error) => {
        end(error)
      }
      await end
      eventSource.close()
    })
  )
}

const subscription = subscriptionExchange({
  enableAllOperations: true,
  forwardSubscription: (request, operation) => ({
    subscribe: (sink) => ({
      unsubscribe: applyAsyncIterableIteratorToSink(
        makeAsyncIterableIteratorFromSink<ExecutionResult>((sink) => {
          const fetchOptions =
            typeof operation.context.fetchOptions === 'function'
              ? operation.context.fetchOptions()
              : operation.context.fetchOptions ?? {}
          const headers = Object.entries(fetchOptions.headers ?? {}).reduce(
            (headers, [key, value]) => ({ ...headers, [key]: value }),
            {} as Record<string, string>
          )
          const searchParams = new URLSearchParams({
            ...headers,
            query: JSON.stringify(request.query ?? ''),
            variables: JSON.stringify(request.variables ?? {})
          })
          const url = new URL(operation.context.url)
          url.search = searchParams.toString()
          const client = makeEventStreamSource(url.toString())
          return applyAsyncIterableIteratorToSink(client, sink)
        }),
        sink
      )
    })
  })
})

const isLiveOperation = (operation: Operation) =>
  (operation.query as DocumentNode).definitions.some((definition) =>
    isLiveQueryOperationDefinitionNode(
      definition,
      operation.variables as { [key: string]: unknown }
    )
  )

export const sseExchange: Exchange = (input) => {
  const forwardSubscription = subscription(input)
  const filterOperation = (operation: Operation) => isLiveOperation(operation)

  return (ops$) => {
    const sharedOps$ = share(ops$)

    const sseResults$ = pipe(
      sharedOps$,
      filter(filterOperation),
      forwardSubscription
    )
    const forward$ = pipe(
      sharedOps$,
      filter((ops) => !filterOperation(ops)),
      input.forward
    )

    return merge([sseResults$, forward$])
  }
}
