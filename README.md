# Beacon - Kubernetes Gateway Token CLI

Get an access token for the Isambard (BriCS) Kubernetes Gateway Service.

Named after the [Bristol Beacon](https://en.wikipedia.org/wiki/Bristol_Beacon), and as beacons are used to assist with navigation.

## Install

Download the binary for your platform from the [releases page](https://github.com/isambard-sc/beacon/releases), make it executable and put it on your `PATH`. For example, on Linux x86_64:

```sh
curl -L -o beacon https://github.com/isambard-sc/beacon/releases/latest/download/beacon-linux-x86_64
chmod +x beacon
mv beacon ~/.local/bin/
```

## Usage

After installing, first run the `auth` command, and follow the link in your browser to login.

```sh
beacon auth             # print a login URL to open in your browser
```

Then run the `token` command to print the access token.

```sh
beacon token            # print the access token
```

For example, to call a service behind the gateway:

```sh
curl https://apps.isambard.ac.uk/k8s-gateway/<project>/<service>/v1/models \
  -H "Authorization: Bearer $(beacon token)"
```

The token can't be refreshed. When it expires, `beacon token` fails and you need to run `beacon auth` again.

## Examples

See [examples/](examples/) for using the token with other tools, such as [opencode](examples/opencode/).
