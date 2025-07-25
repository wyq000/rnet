# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/ci.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/ci.yml)
[![PyPI - License](https://img.shields.io/pypi/l/rnet)](https://github.com/0x676e67/rnet/blob/main/LICENSE)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2F0x676e67%2Frnet%2Fmain%2Fpyproject.toml&logo=python)
[![PyPI](https://img.shields.io/pypi/v/rnet?logo=python)](https://pypi.org/project/rnet/)
[![PyPI Downloads](https://static.pepy.tech/badge/rnet)](https://pepy.tech/projects/rnet)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

A blazing-fast Python HTTP client with advanced browser fingerprinting. Accurately emulates **Chrome**, **Firefox**, **Safari**, **Opera**, and **OkHttp** with precise **TLS/HTTP2** signatures. Powered by [wreq](https://github.com/0x676e67/wreq) for high performance and a clean API.

## Features

- Async and Blocking `Client`s
- Plain bodies, JSON, urlencoded, multipart
- Cookie Store
- Header Order
- Redirect Policy
- Rotating Proxies
- Connection Pooling
- Streaming Transfers
- Zero-Copy Transfers
- WebSocket Upgrade
- Async DNS Resolver
- HTTPS via BoringSSL
- Free-Threaded Safety
- Automatic Decompression

## Example

This asynchronous example demonstrates how to make a simple GET request using the `rnet` library. So you need install `rnet` and run the following code:

```bash
pip install asyncio rnet
```

And then the code:

```python
import asyncio
from rnet import Impersonate, Client


async def main():
    # Build a client
    client = Client(impersonate=Impersonate.Firefox136)

    # Use the API you're already familiar with
    resp = await client.get("https://tls.peet.ws/api/all")
    
    # Print the response
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())

```

Additional learning resources include:

- [DeepWiki](https://deepwiki.com/0x676e67/rnet)
- [API Documentation](https://github.com/0x676e67/rnet/blob/main/rnet.pyi)
- [Repository Tests](https://github.com/0x676e67/rnet/tree/main/tests)
- [Repository Examples](https://github.com/0x676e67/rnet/tree/main/examples)

## Platforms

1. Linux

- **glibc >= 2.34**: `x86_64`, `aarch64`, `armv7`, `i686`
- **musl**: `x86_64`, `aarch64`, `armv7`, `i686`

2. macOS: `x86_64`,`aarch64`

3. Windows: `x86_64`,`i686`,`aarch64`

## Building

1. Install environment

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin
pip install uv

uv venv
source .venv/bin/activate
```

2. Development

```bash
maturin develop --uv
python3 examples/client.py
```

3. Compile wheels

- Local Compilation

Install the BoringSSL build environment by referring to [boring](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
maturin build --release
```

- Musllinux

Make sure the Docker environment is installed. The provided image may be outdated, so you might need to build it yourself. Refer to [rust-cross-musl](https://github.com/0x676e67/toolchain/blob/master/rust-musl-cross/Dockerfile) and the upstream [rust-cross/rust-musl-cross](https://github.com/rust-cross/rust-musl-cross). Note that the upstream image lacks certain platform-specific linker environment variables, which you’ll need to add manually.
  
```bash
bash .github/musl_build.sh x86_64-unknown-linux-musl
bash .github/musl_build.sh aarch64-unknown-linux-musl
bash .github/musl_build.sh armv7-unknown-linux-musleabihf
bash .github/musl_build.sh i686-unknown-linux-musl
```

- Manylinux

For Manylinux compilation, refer to [manylinux](https://github.com/PyO3/maturin?tab=readme-ov-file#manylinux-and-auditwheel).

## Impersonate

In fact, most device models share the same `TLS`/`HTTP2` configuration, with the main difference being the `User-Agent`.

| **Browser**   | **Versions**                                                                                     |
|---------------|--------------------------------------------------------------------------------------------------|
| **Chrome**    | `Chrome100`, `Chrome101`, `Chrome104`, `Chrome105`, `Chrome106`, `Chrome107`, `Chrome108`, `Chrome109`, `Chrome110`, `Chrome114`, `Chrome116`, `Chrome117`, `Chrome118`, `Chrome119`, `Chrome120`, `Chrome123`, `Chrome124`, `Chrome126`, `Chrome127`, `Chrome128`, `Chrome129`, `Chrome130`, `Chrome131`, `Chrome132`, `Chrome133`, `Chrome134`, `Chrome135`, `Chrome136`|
| **Edge**      | `Edge101`, `Edge122`, `Edge127`, `Edge131`, `Edge134`                                                       |
| **Safari**    | `SafariIos17_2`, `SafariIos17_4_1`, `SafariIos16_5`, `Safari15_3`, `Safari15_5`, `Safari15_6_1`, `Safari16`, `Safari16_5`, `Safari17_0`, `Safari17_2_1`, `Safari17_4_1`, `Safari17_5`, `Safari18`, `SafariIPad18`, `Safari18_2`, `SafariIos18_1_1`, `Safari18_3`, `Safari18_3_1` |
| **OkHttp**    | `OkHttp3_9`, `OkHttp3_11`, `OkHttp3_13`, `OkHttp3_14`, `OkHttp4_9`, `OkHttp4_10`, `OkHttp4_12`, `OkHttp5`         |
| **Firefox**   | `Firefox109`, `Firefox117`, `Firefox128`, `Firefox133`, `Firefox135`, `FirefoxPrivate135`, `FirefoxAndroid135`, `Firefox136`, `FirefoxPrivate136`|
| **Opera**    | `Opera116`, `Opera117`, `Opera118`, `Opera119`                                                                 |

## Benchmark

 Faster than `requests`, `httpx`, and `curl_cffi` in most cases — see [benchmark](https://github.com/0x676e67/rnet/tree/main/bench) for details.

## Documentation

For a comprehensive introduction to this library, refer to the [DeepWiki](https://deepwiki.com/0x676e67/rnet) documentation. This AI-generated guide, created by a third party, offers a solid overview and allows interaction with the AI to explore specific APIs.

## Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/0x676e67/rnet/pulls).

## Sponsors

<a href="https://dashboard.capsolver.com/passport/register?inviteCode=y7CtB_a-3X6d" target="_blank"><img src="https://raw.githubusercontent.com/0x676e67/rnet/main/.github/assets/capsolver.jpg" height="47" width="149"></a>

[CapSolver](https://www.capsolver.com/?utm_source=github&utm_medium=banner_repo&utm_campaign=rnet) leverages AI-powered Auto Web Unblock to bypass Captchas effortlessly, providing fast, reliable, and cost-effective data access with seamless integration into Colly, Puppeteer, and Playwright—use code **`RNET`** for a 6% bonus!

## License

**rnet** © [0x676e67](https://github.com/0x676e67), Released under the [GPL-3.0](https://github.com/0x676e67/rnet/blob/main/LICENSE) License.
