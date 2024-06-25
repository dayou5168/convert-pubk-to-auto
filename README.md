# A command line tool for converting a public key to an autonomys style address

## Compile

```shell
cargo build --release
```

## Command line usage

```shell
pubkey-to-auto convert --publickey 0x3ae43b80268XXXXXX81ec6ec2631be705b2e57392aa96b9a37bcc7337a622131
st7bQWbpMe3wVoUtyvtQhDv3f7d9Z7soBnQuTD5gjYykbos1T

# get help
pubkey-to-auto convert --help
Usage: base58_converter convert --publickey <PUBLICKEY>

Options:
  -p, --publickey <PUBLICKEY>
  -h, --help                   Print help
  -V, --version                Print version
```

## run this tool in a server

```shell
pubkey-to-auto convertserver -l 127.0.0.1:4000

# get help
pubkey-to-auto convertserver --help
Listen to a port to start a service

Usage: pubkey-to-auto convertserver --listen <LISTEN>

Options:
  -l, --listen <LISTEN>


  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Notice

This is a tool to convert public key to autonomys address. use the command line or run a server. simple and easy.