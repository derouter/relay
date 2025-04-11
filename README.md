# DeRouter P2P Relay

## Deployment

### Dokku

```sh
dokku apps:create relay
dokku config:set relay SECRET_KEY=
dokku proxy:disable relay
dokku docker-options:add relay deploy "-p 90:5000"
```
