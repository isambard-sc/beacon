# opencode

[opencode.json](opencode.json) is an example [opencode](https://opencode.ai) config for a model served behind the Kubernetes gateway.

## Setup

Copy `opencode.json` into your project or `~/.config/opencode/`, or point `OPENCODE_CONFIG` at it. Then fill in the placeholders:

- `<project>/<service>` in `baseURL`: the service's path on the gateway.
- `<model-id>`: the model's `id`, as returned by `/v1/models`.
- `<model-name>`: any label to show in opencode.

To find the model `id`:

```sh
curl https://apps.isambard.ac.uk/k8s-gateway/<project>/<service>/v1/models \
  -H "Authorization: Bearer $(beacon token)"
```

## Run

The config reads the token from `ISAMBARD_TOKEN`:

```sh
beacon auth
ISAMBARD_TOKEN=$(beacon token) opencode
```

Then pick the model under **Isambard** with `/models`.

opencode reads the token only at startup. When it expires, run `beacon auth` and restart opencode.
