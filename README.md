# Following along with Zero to Production in Rust

see (zero2prod book website)[https://www.zero2prod.com/]

## Running the tests

There are mainly integrations tests which need a database running.
This is a docker instance which gets started with the script in
*scripts/init-db.sh* which starts the database and applies the
database migrations.

In order to find the database locally we need 2 environment variables

- DATABASE_URL
- APP_ENVIRONMENT

which are set in the *.env* file checked in and set for local dev.

    $ cargo test

To enable logging to the screen set the `TEST_LOG` variable

    $ TESTLOG=1 cargo test


