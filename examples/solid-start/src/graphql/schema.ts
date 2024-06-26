import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core'
export type Maybe<T> = T | null
export type InputMaybe<T> = Maybe<T>
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K]
}
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>
}
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>
}
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string
  String: string
  Boolean: boolean
  Int: number
  Float: number
  DateTime: any
}

export type Mutation = {
  __typename?: 'Mutation'
  /** Create a Todo */
  todoCreate?: Maybe<TodoCreatePayload>
  /** Delete a Todo by ID or unique field */
  todoDelete?: Maybe<TodoDeletePayload>
  /** Create a TodoList */
  todoListCreate?: Maybe<TodoListCreatePayload>
  /** Delete a TodoList by ID or unique field */
  todoListDelete?: Maybe<TodoListDeletePayload>
  /** Update a TodoList */
  todoListUpdate?: Maybe<TodoListUpdatePayload>
  /** Update a Todo */
  todoUpdate?: Maybe<TodoUpdatePayload>
}

export type MutationTodoCreateArgs = {
  input: TodoCreateInput
}

export type MutationTodoDeleteArgs = {
  by: TodoByInput
}

export type MutationTodoListCreateArgs = {
  input: TodoListCreateInput
}

export type MutationTodoListDeleteArgs = {
  by: TodoListByInput
}

export type MutationTodoListUpdateArgs = {
  by: TodoListByInput
  input: TodoListUpdateInput
}

export type MutationTodoUpdateArgs = {
  by: TodoByInput
  input: TodoUpdateInput
}

export type PageInfo = {
  __typename?: 'PageInfo'
  endCursor?: Maybe<Scalars['String']>
  hasNextPage: Scalars['Boolean']
  hasPreviousPage: Scalars['Boolean']
  startCursor?: Maybe<Scalars['String']>
}

export type Query = {
  __typename?: 'Query'
  /** Query a single Todo by an ID or a unique field */
  todo?: Maybe<Todo>
  /** Paginated query to fetch the whole list of `Todo`. */
  todoCollection?: Maybe<TodoConnection>
  /** Query a single TodoList by an ID or a unique field */
  todoList?: Maybe<TodoList>
  /** Paginated query to fetch the whole list of `TodoList`. */
  todoListCollection?: Maybe<TodoListConnection>
}

export type QueryTodoArgs = {
  by: TodoByInput
}

export type QueryTodoCollectionArgs = {
  after?: InputMaybe<Scalars['String']>
  before?: InputMaybe<Scalars['String']>
  first?: InputMaybe<Scalars['Int']>
  last?: InputMaybe<Scalars['Int']>
}

export type QueryTodoListArgs = {
  by: TodoListByInput
}

export type QueryTodoListCollectionArgs = {
  after?: InputMaybe<Scalars['String']>
  before?: InputMaybe<Scalars['String']>
  first?: InputMaybe<Scalars['Int']>
  last?: InputMaybe<Scalars['Int']>
}

export type Todo = {
  __typename?: 'Todo'
  complete: Scalars['Boolean']
  /** when the model was created */
  createdAt: Scalars['DateTime']
  id: Scalars['ID']
  list?: Maybe<TodoList>
  title: Scalars['String']
  /** when the model was updated */
  updatedAt: Scalars['DateTime']
}

export type TodoByInput = {
  id?: InputMaybe<Scalars['ID']>
}

export type TodoConnection = {
  __typename?: 'TodoConnection'
  edges?: Maybe<Array<Maybe<TodoEdge>>>
  /** Information to aid in pagination */
  pageInfo: PageInfo
}

/** Input to create a new Todo */
export type TodoCreateInput = {
  complete: Scalars['Boolean']
  list?: InputMaybe<TodoTodoRelateTodoListTodoListCreateRelationInput>
  title: Scalars['String']
}

export type TodoCreatePayload = {
  __typename?: 'TodoCreatePayload'
  todo?: Maybe<Todo>
}

export type TodoDeletePayload = {
  __typename?: 'TodoDeletePayload'
  deletedId: Scalars['ID']
}

export type TodoEdge = {
  __typename?: 'TodoEdge'
  cursor: Scalars['String']
  node: Todo
}

export type TodoList = {
  __typename?: 'TodoList'
  /** when the model was created */
  createdAt: Scalars['DateTime']
  id: Scalars['ID']
  title: Scalars['String']
  todos?: Maybe<TodoConnection>
  /** when the model was updated */
  updatedAt: Scalars['DateTime']
}

export type TodoListTodosArgs = {
  after?: InputMaybe<Scalars['String']>
  before?: InputMaybe<Scalars['String']>
  first?: InputMaybe<Scalars['Int']>
  last?: InputMaybe<Scalars['Int']>
}

export type TodoListByInput = {
  id?: InputMaybe<Scalars['ID']>
}

export type TodoListConnection = {
  __typename?: 'TodoListConnection'
  edges?: Maybe<Array<Maybe<TodoListEdge>>>
  /** Information to aid in pagination */
  pageInfo: PageInfo
}

/** Input to create a new TodoList */
export type TodoListCreateInput = {
  title: Scalars['String']
  todos?: InputMaybe<
    Array<InputMaybe<TodoListTodoRelateTodoListTodoCreateRelationInput>>
  >
}

export type TodoListCreatePayload = {
  __typename?: 'TodoListCreatePayload'
  todoList?: Maybe<TodoList>
}

export type TodoListDeletePayload = {
  __typename?: 'TodoListDeletePayload'
  deletedId: Scalars['ID']
}

export type TodoListEdge = {
  __typename?: 'TodoListEdge'
  cursor: Scalars['String']
  node: TodoList
}

/** Input to create a new TodoListTodoRelateTodoListTodo */
export type TodoListTodoRelateTodoListTodoCreateInput = {
  complete: Scalars['Boolean']
  title: Scalars['String']
}

/** Input to create a new TodoListTodoRelateTodoListTodo relation */
export type TodoListTodoRelateTodoListTodoCreateRelationInput = {
  create?: InputMaybe<TodoListTodoRelateTodoListTodoCreateInput>
  link?: InputMaybe<Scalars['ID']>
}

/** Input to update a TodoListTodoRelateTodoListTodo relation */
export type TodoListTodoRelateTodoListTodoUpdateRelationInput = {
  create?: InputMaybe<TodoListTodoRelateTodoListTodoCreateInput>
  link?: InputMaybe<Scalars['ID']>
  unlink?: InputMaybe<Scalars['ID']>
}

/** Input to create a new TodoList */
export type TodoListUpdateInput = {
  title?: InputMaybe<Scalars['String']>
  todos?: InputMaybe<
    Array<InputMaybe<TodoListTodoRelateTodoListTodoUpdateRelationInput>>
  >
}

export type TodoListUpdatePayload = {
  __typename?: 'TodoListUpdatePayload'
  todoList?: Maybe<TodoList>
}

/** Input to create a new TodoTodoRelateTodoListTodoList */
export type TodoTodoRelateTodoListTodoListCreateInput = {
  title: Scalars['String']
}

/** Input to create a new TodoTodoRelateTodoListTodoList relation */
export type TodoTodoRelateTodoListTodoListCreateRelationInput = {
  create?: InputMaybe<TodoTodoRelateTodoListTodoListCreateInput>
  link?: InputMaybe<Scalars['ID']>
}

/** Input to update a TodoTodoRelateTodoListTodoList relation */
export type TodoTodoRelateTodoListTodoListUpdateRelationInput = {
  create?: InputMaybe<TodoTodoRelateTodoListTodoListCreateInput>
  link?: InputMaybe<Scalars['ID']>
  unlink?: InputMaybe<Scalars['ID']>
}

/** Input to create a new Todo */
export type TodoUpdateInput = {
  complete?: InputMaybe<Scalars['Boolean']>
  list?: InputMaybe<TodoTodoRelateTodoListTodoListUpdateRelationInput>
  title?: InputMaybe<Scalars['String']>
}

export type TodoUpdatePayload = {
  __typename?: 'TodoUpdatePayload'
  todo?: Maybe<Todo>
}

export type TodoFragment = {
  __typename?: 'Todo'
  id: string
  title: string
  complete: boolean
}

export type TodoListFragment = {
  __typename?: 'TodoList'
  id: string
  title: string
  todos?: {
    __typename?: 'TodoConnection'
    edges?: Array<{
      __typename?: 'TodoEdge'
      node: {
        __typename?: 'Todo'
        id: string
        title: string
        complete: boolean
      }
    } | null> | null
  } | null
}

export type TodoListsQueryVariables = Exact<{ [key: string]: never }>

export type TodoListsQuery = {
  __typename?: 'Query'
  todoListCollection?: {
    __typename?: 'TodoListConnection'
    edges?: Array<{
      __typename?: 'TodoListEdge'
      node: {
        __typename?: 'TodoList'
        id: string
        title: string
        todos?: {
          __typename?: 'TodoConnection'
          edges?: Array<{
            __typename?: 'TodoEdge'
            node: {
              __typename?: 'Todo'
              id: string
              title: string
              complete: boolean
            }
          } | null> | null
        } | null
      }
    } | null> | null
  } | null
}

export type TodoListCreateMutationVariables = Exact<{
  title: Scalars['String']
}>

export type TodoListCreateMutation = {
  __typename?: 'Mutation'
  todoListCreate?: {
    __typename?: 'TodoListCreatePayload'
    todoList?: {
      __typename?: 'TodoList'
      id: string
      title: string
      todos?: {
        __typename?: 'TodoConnection'
        edges?: Array<{
          __typename?: 'TodoEdge'
          node: {
            __typename?: 'Todo'
            id: string
            title: string
            complete: boolean
          }
        } | null> | null
      } | null
    } | null
  } | null
}

export type TodoCreateMutationVariables = Exact<{
  title: Scalars['String']
  todoListId: Scalars['ID']
}>

export type TodoCreateMutation = {
  __typename?: 'Mutation'
  todoCreate?: {
    __typename?: 'TodoCreatePayload'
    todo?: {
      __typename?: 'Todo'
      id: string
      title: string
      complete: boolean
    } | null
  } | null
}

export type TodoListDeleteMutationVariables = Exact<{
  id: Scalars['ID']
}>

export type TodoListDeleteMutation = {
  __typename?: 'Mutation'
  todoListDelete?: {
    __typename?: 'TodoListDeletePayload'
    deletedId: string
  } | null
}

export type TodoListUpdateMutationVariables = Exact<{
  id: Scalars['ID']
  title?: InputMaybe<Scalars['String']>
}>

export type TodoListUpdateMutation = {
  __typename?: 'Mutation'
  todoListUpdate?: {
    __typename?: 'TodoListUpdatePayload'
    todoList?: {
      __typename?: 'TodoList'
      id: string
      title: string
      todos?: {
        __typename?: 'TodoConnection'
        edges?: Array<{
          __typename?: 'TodoEdge'
          node: {
            __typename?: 'Todo'
            id: string
            title: string
            complete: boolean
          }
        } | null> | null
      } | null
    } | null
  } | null
}

export type TodoDeleteMutationVariables = Exact<{
  id: Scalars['ID']
}>

export type TodoDeleteMutation = {
  __typename?: 'Mutation'
  todoDelete?: { __typename?: 'TodoDeletePayload'; deletedId: string } | null
}

export type TodoUpdateMutationVariables = Exact<{
  id: Scalars['ID']
  title?: InputMaybe<Scalars['String']>
  complete?: InputMaybe<Scalars['Boolean']>
}>

export type TodoUpdateMutation = {
  __typename?: 'Mutation'
  todoUpdate?: {
    __typename?: 'TodoUpdatePayload'
    todo?: {
      __typename?: 'Todo'
      id: string
      title: string
      complete: boolean
    } | null
  } | null
}

export const TodoFragmentDoc = {
  kind: 'Document',
  definitions: [
    {
      kind: 'FragmentDefinition',
      name: { kind: 'Name', value: 'Todo' },
      typeCondition: {
        kind: 'NamedType',
        name: { kind: 'Name', value: 'Todo' }
      },
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          { kind: 'Field', name: { kind: 'Name', value: 'id' } },
          { kind: 'Field', name: { kind: 'Name', value: 'title' } },
          { kind: 'Field', name: { kind: 'Name', value: 'complete' } }
        ]
      }
    }
  ]
} as unknown as DocumentNode<TodoFragment, unknown>
export const TodoListFragmentDoc = {
  kind: 'Document',
  definitions: [
    {
      kind: 'FragmentDefinition',
      name: { kind: 'Name', value: 'TodoList' },
      typeCondition: {
        kind: 'NamedType',
        name: { kind: 'Name', value: 'TodoList' }
      },
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          { kind: 'Field', name: { kind: 'Name', value: 'id' } },
          { kind: 'Field', name: { kind: 'Name', value: 'title' } },
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todos' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'first' },
                value: { kind: 'IntValue', value: '100' }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'edges' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'Field',
                        name: { kind: 'Name', value: 'node' },
                        selectionSet: {
                          kind: 'SelectionSet',
                          selections: [
                            {
                              kind: 'FragmentSpread',
                              name: { kind: 'Name', value: 'Todo' }
                            }
                          ]
                        }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoFragmentDoc.definitions
  ]
} as unknown as DocumentNode<TodoListFragment, unknown>
export const TodoListsDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'query',
      name: { kind: 'Name', value: 'TodoLists' },
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoListCollection' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'first' },
                value: { kind: 'IntValue', value: '100' }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'edges' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'Field',
                        name: { kind: 'Name', value: 'node' },
                        selectionSet: {
                          kind: 'SelectionSet',
                          selections: [
                            {
                              kind: 'FragmentSpread',
                              name: { kind: 'Name', value: 'TodoList' }
                            }
                          ]
                        }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoListFragmentDoc.definitions
  ]
} as unknown as DocumentNode<TodoListsQuery, TodoListsQueryVariables>
export const TodoListCreateDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoListCreate' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'title' }
          },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'String' } }
          }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoListCreate' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'input' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'title' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'title' }
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'todoList' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'FragmentSpread',
                        name: { kind: 'Name', value: 'TodoList' }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoListFragmentDoc.definitions
  ]
} as unknown as DocumentNode<
  TodoListCreateMutation,
  TodoListCreateMutationVariables
>
export const TodoCreateDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoCreate' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'title' }
          },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'String' } }
          }
        },
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'todoListId' }
          },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'ID' } }
          }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoCreate' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'input' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'title' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'title' }
                      }
                    },
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'complete' },
                      value: { kind: 'BooleanValue', value: false }
                    },
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'list' },
                      value: {
                        kind: 'ObjectValue',
                        fields: [
                          {
                            kind: 'ObjectField',
                            name: { kind: 'Name', value: 'link' },
                            value: {
                              kind: 'Variable',
                              name: { kind: 'Name', value: 'todoListId' }
                            }
                          }
                        ]
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'todo' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'FragmentSpread',
                        name: { kind: 'Name', value: 'Todo' }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoFragmentDoc.definitions
  ]
} as unknown as DocumentNode<TodoCreateMutation, TodoCreateMutationVariables>
export const TodoListDeleteDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoListDelete' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: { kind: 'Variable', name: { kind: 'Name', value: 'id' } },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'ID' } }
          }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoListDelete' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'by' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'id' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'id' }
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                { kind: 'Field', name: { kind: 'Name', value: 'deletedId' } }
              ]
            }
          }
        ]
      }
    }
  ]
} as unknown as DocumentNode<
  TodoListDeleteMutation,
  TodoListDeleteMutationVariables
>
export const TodoListUpdateDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoListUpdate' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: { kind: 'Variable', name: { kind: 'Name', value: 'id' } },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'ID' } }
          }
        },
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'title' }
          },
          type: { kind: 'NamedType', name: { kind: 'Name', value: 'String' } }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoListUpdate' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'by' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'id' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'id' }
                      }
                    }
                  ]
                }
              },
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'input' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'title' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'title' }
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'todoList' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'FragmentSpread',
                        name: { kind: 'Name', value: 'TodoList' }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoListFragmentDoc.definitions
  ]
} as unknown as DocumentNode<
  TodoListUpdateMutation,
  TodoListUpdateMutationVariables
>
export const TodoDeleteDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoDelete' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: { kind: 'Variable', name: { kind: 'Name', value: 'id' } },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'ID' } }
          }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoDelete' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'by' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'id' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'id' }
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                { kind: 'Field', name: { kind: 'Name', value: 'deletedId' } }
              ]
            }
          }
        ]
      }
    }
  ]
} as unknown as DocumentNode<TodoDeleteMutation, TodoDeleteMutationVariables>
export const TodoUpdateDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: { kind: 'Name', value: 'TodoUpdate' },
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: { kind: 'Variable', name: { kind: 'Name', value: 'id' } },
          type: {
            kind: 'NonNullType',
            type: { kind: 'NamedType', name: { kind: 'Name', value: 'ID' } }
          }
        },
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'title' }
          },
          type: { kind: 'NamedType', name: { kind: 'Name', value: 'String' } }
        },
        {
          kind: 'VariableDefinition',
          variable: {
            kind: 'Variable',
            name: { kind: 'Name', value: 'complete' }
          },
          type: { kind: 'NamedType', name: { kind: 'Name', value: 'Boolean' } }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: { kind: 'Name', value: 'todoUpdate' },
            arguments: [
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'by' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'id' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'id' }
                      }
                    }
                  ]
                }
              },
              {
                kind: 'Argument',
                name: { kind: 'Name', value: 'input' },
                value: {
                  kind: 'ObjectValue',
                  fields: [
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'title' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'title' }
                      }
                    },
                    {
                      kind: 'ObjectField',
                      name: { kind: 'Name', value: 'complete' },
                      value: {
                        kind: 'Variable',
                        name: { kind: 'Name', value: 'complete' }
                      }
                    }
                  ]
                }
              }
            ],
            selectionSet: {
              kind: 'SelectionSet',
              selections: [
                {
                  kind: 'Field',
                  name: { kind: 'Name', value: 'todo' },
                  selectionSet: {
                    kind: 'SelectionSet',
                    selections: [
                      {
                        kind: 'FragmentSpread',
                        name: { kind: 'Name', value: 'Todo' }
                      }
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    ...TodoFragmentDoc.definitions
  ]
} as unknown as DocumentNode<TodoUpdateMutation, TodoUpdateMutationVariables>
