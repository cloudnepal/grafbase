import { graph, connector, config } from '@grafbase/sdk'

const g = graph.Standalone()

const openapi = connector.OpenAPI('OpenAPI', {
  schema: g.env('SCHEMA_URL')
})

g.datasource(openapi, { namespace: false })

export default config({
  graph: g,
  federation: { version: '2.3' },
  auth: {
    rules: (rules) => {
      rules.public()
    }
  }
})
