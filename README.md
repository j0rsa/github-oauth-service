# Github OAuth Authentication service

This is a simple service to authenticate users via Github OAuth and use it as an authentication middleware in a bundle with Traefik

[Traefik setup](https://doc.traefik.io/traefik/v1.7/configuration/backends/kubernetes/#authentication)

[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)

[![Base Docker image scratch](https://img.shields.io/badge/Base%20Image-Scratch-blue)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![Image Size](https://img.shields.io/badge/image%20size-9.85MB-green)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![MicroBadger Layers](https://img.shields.io/microbadger/layers/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)

[![CodeFactor](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/badge/master)](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/overview/master)

## Endpoints
| Method | URL | Description |
| ------:| --- | ----------- |
| `GET` | `/health` | Healthcheck  which returns Code 200|
| `GET` | `/auth/login` | Redirect to login page with required scopes for provided client id |
| `POST` | `/auth/token` | Get JWT token by passing user code `{ "code": "<code>"}` after auth on https://github.com/login/oauth/authorize?scope=user%3Aread,user%3Aemail&client_id=... |
| `POST` | `/auth/refresh` | Refresh token with a new one by passing the old valid one `{ "token": "eyJhbGciOiJIUz..." }` |
| `POST` | `/auth/check` | Checks the token and returns code 200 with Headers: `X-Auth-Id` with user id, `X-Auth-User` with user name and `X-Github-Token` with github oauth user token|

## Environment variables
| Variable | Default value | Description |
| ------| --- | ----------- |
| RUST_LOG | info | defines the log level of app |
| BIND_ADDRESS | 0.0.0.0 | Address of web server to listen connections |
| BIND_PORT | 8080 | Port of web server to listen connections |
| **JWT_SECRET** | -- | JWT HS256 Secret Key |
| JWT_ISS | "" | iss (issuer): Issuer of the JWT |
| JWT_AUD | "" | aud (audience): Recipient for which the JWT is intended |
| JWT_EXP_DAYS | 30 | exp (expiration time): Time in days after which the JWT expires |
| JWT_NBF_DAYS | 0 | nbf (not before time): Time in days before which the JWT must not be accepted for processing |
| JWT_LEEWAY_SEC | 0 | leeway (in seconds) to the `exp`, `iat` and `nbf` validation to  account for clock skew |
| GH_SCOPE | "user:read,user:email" | Github Scope to request |
| **GH_CLIENT_ID** | "" | Github oAuth App client id |
| **GH_CLIENT_SECRET** | "" | Github oAuth App client secret | 
| **GH_CODE_REDIRECT** | "" | Redirect page after login |

*Bold variables are required to specify 

# Build

## Build release locally
    cargo build --release

## Build release in docker and prepare an image
    docker build -t j0rsa/jwt-auth .
    
ref: https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

ref: https://medium.com/@gdiener/how-to-build-a-smaller-docker-image-76779e18d48a

# Troubleshooting

## Inspect image filesystem
    docker run --rm -it <image name or id> sh
## Test run
    docker run --rm -it jwt-auth