<h1 align="center">timet-rs</h1>
<p align="center">
    <a href="https://github.com/sondr3/timet-rs/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/timet-rs/workflows/pipeline/badge.svg" /></a>
    <a href="https://crates.io/crates/timet-rs"><img alt="Crates" src="https://img.shields.io/crates/v/timet-rs.svg" /></a>
</p>

<p align="center">
    <b>Automate your hours</b>
</p>

- **Simple**: `timet --init` to create a config
- **Handy**: `timet` to fetch your hours for the current month

## Usage

Simply run `timet` to get your hours for the current month. 

```shell
$ timet
```


If you want to get the hours for a different month, you can use the `-m` and `-y` flags to specify the month and year.

```shell
$ timet -m 1 -y 2021  # Get the hours for January 2021
```

## Completion

If your method of installation didn't include shell completion, you can manually
source or save them with the `timet --completion <shell>` command.

## Help

```shell
$ timet --help
Usage: timet [OPTIONS] --completions <COMPLETIONS>

Options:
  -m, --month <MONTH>              Month to get the time entries for, defaults to this month
  -y, --year <YEAR>                Year to get the time entries for, defaults to this year
  -i, --init                       Create a new config file
      --completions <COMPLETIONS>  Create shell completions [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                       Print help
  -V, --version                    Print version
```


# Installation

Currently, the package is available a couple of places, including Homebrew, AUR and Nix.

<dl>
  <dt>Cargo</dt>
  <dd><code>cargo install --locked timet-rs</code></dd>

  <dt>Homebrew</dt>
  <dd>
    <ol>
      <li><code>brew tap sondr3/homebrew-taps</code></li>
      <li><code>brew install sondr3/homebrew-taps/timet</code></li>
    <ol>
  </dd>
</dl>

## Release pages

You can also download the matching release from the [release
tab](https://github.com/sondr3/timet-rs/releases), extracting the archive and
placing the binary in your `$PATH`. Note that for Linux the
`unknown-linux-musl.tar.gz` is preferred as it is statically linked and thus
should run on any Linux distribution.

# LICENSE

WTFPL
