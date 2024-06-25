# A command line tool for converting a public key to atonomys style adreess

## Compile

```shell
cargo build --release
```

## Usage

```shell
add-convert.exe convert --publickey 0x3ae43b80268XXXXXX81ec6ec2631be705b2e57392aa96b9a37bcc7337a622131
st7bQWbpMe3wVoUtyvtQhDv3f7d9Z7soBnQuTD5gjYykbos1T

# get help
add-convert.exe convert --help
Usage: base58_converter convert --publickey <PUBLICKEY>

Options:
  -p, --publickey <PUBLICKEY>
  -h, --help                   Print help
  -V, --version                Print version
```

## Notice

It's a tool copied and do some tweaks to simplify the convert process. 