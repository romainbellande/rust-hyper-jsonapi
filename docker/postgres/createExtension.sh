#!/usr/bin/env bash

# gosu $POSTGRES_USER $POSTGRES_PASSWORD --single <<- EOSQL
#    CREATE EXTENSION IF NOT EXISTS "pgcrypto";
# EOSQL
psql -U $POSTGRES_USER -w jsonapi_rust -c "CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";"
