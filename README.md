# Beacon - Kubernetes Gateway Service Token CLI

Get an access token for the Isambard (BriCS) Kubernetes Gateway Service.

Named after the [Bristol Beacon](https://en.wikipedia.org/wiki/Bristol_Beacon), and as beacons are used to assist with navigation.

## Install

Download the binary for your platform from the [releases page](https://github.com/isambard-sc/beacon/releases), make it executable and put it on your `PATH`. For example, on Linux x86_64:

```sh
curl -L -o beacon https://github.com/isambard-sc/beacon/releases/latest/download/beacon-linux-x86_64
chmod +x beacon
mv beacon ~/.local/bin/
```

For other platforms, replace `beacon-linux-x86_64` with:

| Platform | File |
| --- | --- |
| Linux x86_64 | `beacon-linux-x86_64` |
| Linux aarch64 | `beacon-linux-aarch64` |
| Linux x86_64 (static, musl) | `beacon-linux-musl-x86_64` |
| Linux aarch64 (static, musl) | `beacon-linux-musl-aarch64` |
| macOS Apple silicon | `beacon-macos-aarch64` |
| macOS Intel | `beacon-macos-x86_64` |
| Windows x86_64 | `beacon-windows-x86_64.exe` |
| FreeBSD x86_64 | `beacon-freebsd-x86_64` |

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

To check who you're logged in as, or to remove the cached token:

```sh
beacon whoami           # show the claims in the cached token
beacon logout           # delete the cached token
```

## Examples

- [opencode](examples/opencode/): use a model served behind the gateway.
