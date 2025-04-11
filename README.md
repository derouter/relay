# DeRouter P2P Relay

## Deployment

### Dokku

```sh
# Set the Dokku app name for the relay.
export RELAY_APP_NAME=relay

# ⚠️ Set the host's port for the relay.
export RELAY_HOST_PORT=

# ⚠️ Relay's peer ID is derived from this secret.
# Hex-encoded 32-bytes-long string.
export RELAY_SECRET_KEY=

dokku apps:create $RELAY_APP_NAME
dokku config:set $RELAY_APP_NAME SECRET_KEY=$RELAY_SECRET_KEY

# Disable Dokku proxy because it doesn't support TCP proxying,
# and forward host's $RELAY_HOST_PORT to the container's port 5000.
# See https://dokku.com/docs/networking/port-management.
dokku proxy:disable $RELAY_APP_NAME
dokku docker-options:add $RELAY_APP_NAME deploy "-p $RELAY_HOST_PORT:5000"

# Disable zero-downtime deployment to avoid same port binding issue.
# See https://dokku.com/docs/deployment/zero-downtime-deploys.
dokku checks:disable $RELAY_APP_NAME
```
