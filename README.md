# bloom

Simple binary that runs a configurable bloom filter over lines / tokens from STDIN.

```
Usage: bloom [OPTIONS]

Options:
  -b, --bitmap-bytes <BITMAP_BYTES>  [default: 16384]
  -c, --count <COUNT>                [default: 16384]
  -d, --delimiter <DELIMITER>        
  -i, --index <INDEX>                
  -h, --help                         Print help
  -V, --version                      Print version
```

Examples:

Simple line filter:

```sh
$ cat << EOF | bloom
foo
bar
baz
bar
EOF
foo
bar
baz
```

Filter by delimited token and index position:

```sh
$ cat << EOF | bloom -d ';' -i 1
1;2
2;3
3;4
5;2
EOF
1;2
2;3
3;4
```
