# Contributing

## Build the charm

Build the charm in this git repository using:

```shell
charmcraft pack
```

## Deploy the charm

```shell
# On AMD64.
juju deploy ./qdrant-k8s_ubuntu@24.04-amd64.charm.charm --resource qdrant-image=qdrant/qdrant:v1.13.6

# On ARM64.
juju deploy ./qdrant-k8s_ubuntu@24.04-arm64.charm.charm --resource qdrant-image=qdrant/qdrant:v1.13.6
```

## Test the charm

```shell
curl http://<qdrant-unit-ip>:6333/collections
```
