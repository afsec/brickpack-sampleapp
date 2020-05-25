# BrickPack SampleApp

A sample CNA (Cloud Native Application)/Microservice writen using brickpack: https://github.com/afsec/brickpack

## Lean artifact (~2 MB)
The whole microservice is compiled as statically compiled via MUSL target.
```
$ ls -lh dist/
total 2,0M
-rwxrwxr-x 1 user user 2,0M mai 20 00:33 brickpack-sampleapp

```

## Startup message
```
$ CLIENT_TOKEN="SomeRandomToken" ./dist/brickpack-sampleapp
Starting App [brickpack-sampleapp v0.1.6]:

Brickpack Web Framework v0.3.0

System Endpoints:
                       GET   - /
                       GET   - /auth
                       PATCH - /maintenance

Application Endpoints:
                       POST  - /api/show-users
                       POST  - /api/show-posts


CLIENT_TOKEN: SomeRandomToken

Listening at: http://0.0.0.0:8000

```
