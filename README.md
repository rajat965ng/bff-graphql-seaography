# Seaography

<p>
Seaography is a GraphQL framework and code generator for SeaORM
</p>

## Introduction
Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities. It ships with a CLI tool that can generate ready-to-compile Rust GraphQL servers from existing MySQL, Postgres and SQLite databases.

## Benefits
- Quick and easy to get started
- Generates readable code
- Extensible project structure
- Based on popular async libraries: async-graphql  and SeaORM

## Features
- Relational query (1-to-1, 1-to-N)
- Pagination on query's root entity
- Filter with operators (e.g. gt, lt, eq)
- Order by any column

## Content of pgpass file
`hostname:port:database:username:password`

## Postgres with PgAdmin start
`podman-compose up`

![](images/img_2.png)

## Generate GraphQL code
`seaography-cli postgres://postgres:postgres@localhost:5432/employee employee employee`

![](images/img_1.png)

## Query employee data

![](images/img.png)
