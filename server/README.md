CodeChain Dashboard Server
==========================

[![Gitter](https://badges.gitter.im/CodeChain-io.svg)](https://gitter.im/CodeChain-io/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

CodeChain Dashboard Server is a server which collects many CodeChain node's information(best block, pending transactions, log, ...). Also, CodeChain Dashboard Server serves collected data to CodeChain Dashboard.

Install
--------

You should set up a rust toolchain.

You can install codechain-dashboard-server by running `cargo install`

Install Postgres and create schema
-----------------

Ubuntu
```
sudo apt install postgresql postgresql-contrib
sudo -u postgres psql -f create_user_and_db.sql
generate-schema
```

Mac (brew)
```
brew install postgresql
brew services start postgresql
psql postgres -f create_user_and_db.sql
generate-schema
```

Run
----

Just run `codechain-dashboard-server` in your shell.
To safely communicate with the Dashboard, please set the `PASSPHRASE` environment variable. The Dashboard program should use the same passphrase.
Also, you should set `NETWORK_ID` environment variable to print the network id in log messages.

When you are using the `PASSPHRASE` you should use SSL over the connection. If you don't use the SSL, the `PASSPHRASE` is open to the internet. 

CodeChain Dashboard Server will listen 3012 port to communicate with the Dashboard using JSON-RPC.

CodeChain Dashboard Server will listen 4012 port to communicate with the Dashboard Client using JSON-RPC.

Alerts
-------

The server sends an alert via Slack and Email in situations where there likely is a problem.

## Email alerts
To use email alerts, the server needs the [Sendgird](https://sendgrid.com/) api key.
```
SENDGRID_API_KEY={api key} SENDGRID_TO={email address} codechain-dashboard-server
```

## Slack alerts
The server uses [webhooks](https://api.slack.com/incoming-webhooks)
```
SLACK_HOOK_URL={web hook url} codechain-dashboard-server
```

Environmental Variables
------------------------

| NAME                | DESCRIPTION                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------ |
| START_AT_CONNECT    | If this variable is set, a CodeChain instance is started once a dashboard client connects to the dashboard server. |
| NETWORK_ID          | Network ID information that is used in error messages or logs.                                                     |
| SLACK_WEBHOOK_URL   | Used to send alarms to Slack.                                                                                      |
| SENDGRID_TO         | An email address to receive alarm emails.                                                                          |
| SENDGRID_API_KEY    | An API Key that is used to send alarms.                                                                            |
| PASSPHRASE          | A passphrase that is used to communicate with the Dashboard safely.                                                |
| ENABLE_MEMORY_ALARM | When this variable is set, the Dashboard Server sends memory alarms.                                               |
