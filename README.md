# BrickPack SampleApp

A sample CNA (Cloud Native Application)/Microservice writen using brickpack: https://github.com/afsec/brickpack

## Lean artifact (~7 MB)
The whole microservice is built with static compiling using **MUSL** target.
```
$ ls -lh
total 6,5M
-rwxrwxr-x 1 user user 6,5M mai 30 18:09 brickpack-sampleapp
```

## Startup message
```
$ BP_SERVER_TOKEN="SomeRandomToken" DB_SERVER_URL="http://concierge-db:3341" BP_CLIENT_TOKEN="SomeAnotherRandomToken" ./dist/brickpack-sampleapp
$ BP_SERVER_TOKEN="SomeRandomToken" ./dist/brickpack-sampleapp
{"level":30,"time":1590873126295,"msg":"Logger started","level":"Info"}
{"level":30,"time":1590873126295,"msg":"Starting App [brickpack-sampleapp v0.2.5]:"}
{"level":30,"time":1590873126295,"msg":"Powered by Brickpack Web Framework v0.7.8"}
{"level":30,"time":1590873126295,"msg":"System Endpoints:"}
{"level":30,"time":1590873126295,"msg":"                       GET   - /"}
{"level":30,"time":1590873126295,"msg":"                       GET   - /auth"}
{"level":30,"time":1590873126295,"msg":"                       PATCH - /maintenance"}
{"level":30,"time":1590873126295,"msg":"Application Endpoints:"}
{"level":30,"time":1590873126295,"msg":"                       POST  - /api/show-posts"}
{"level":30,"time":1590873126295,"msg":"                       POST  - /api/show-users"}
{"level":30,"time":1590873126296,"msg":"BP_SERVER_TOKEN: SomeRandomToken"}
{"level":30,"time":1590873126296,"msg":"Server listening","address":"http://0.0.0.0:8000","target":"release","tls":false}
```
