# BrickPack SampleApp

A sample CNA (Cloud Native Application)/Microservice writen using brickpack: https://github.com/afsec/brickpack

## Lean artifact (~5 MB)
The whole microservice is built with static compiling using **MUSL** target.
```
$ ls -lh
total 5,0M
-rwxrwxr-x 1 user user 5,0M mai 27 16:55 brickpack-sampleapp

```

## Startup message
```
$ BP_SERVER_TOKEN="SomeRandomToken" DB_SERVER_URL="http://concierge-db:3341" BP_CLIENT_TOKEN="SomeAnotherRandomToken" ./dist/brickpack-sampleapp
Starting App [brickpack-sampleapp v0.2.0]:

Brickpack Web Framework v0.6.2

System Endpoints:
                       GET   - /
                       GET   - /auth
                       PATCH - /maintenance

Application Endpoints:
                       POST  - /api/show-users
                       POST  - /api/show-posts


BP_SERVER_TOKEN: SomeRandomToken

Listening at: http://0.0.0.0:8000

```
