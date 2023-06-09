# `syntastica-parsers-git`

Collection of tree-sitter parsers for
[`syntastica`](https://crates.io/crates/syntastica), fetching with git in the
build-script.

See
[the project overview](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
for more information on all parser collections.

## Improving compilation speed

Due to the nature of this parser collection, compilation can take a very long
time with many parsers enabled, since all parsers are cloned and built every
time the build script is run. In a local development environment you can use the
following environment variables to reuse files from a previous run.

### Option one: Setting a different clone directory

You can set `SYNTASTICA_PARSERS_CLONE_DIR` to a directory to clone the parser
repositories to, so that they only have to be built but not downloaded every
time.

### Option two: Reusing built archives

Run the build once with `SYNTASTICA_PARSERS_CACHE_WRITE_DIR` set to a free
directory once. This will copy all built parser binaries to the specified
directory. For all following runs you can then set
`SYNTASTICA_PARSERS_CACHE_READ_DIR` to the same directory and the build script
will reuse these binary files.

<!-- Everything under here is autogenerated by running `cargo xtask codegen` -->
<!-- DO NOT EDIT! -->

## List of included parsers

<!-- dprint-ignore-start -->

<details>
<summary>List of parsers included in the <span class="stab portability"><code>some</code></span> feature</summary>

- [bash](https://github.com/tree-sitter/tree-sitter-bash/tree/ee2a8f9906b53a785b784ee816c0016c2b6866d2)
- [c](https://github.com/tree-sitter/tree-sitter-c/tree/cac392ac3d7d365c469971b117e92a0df3bc8305)
- [cpp](https://github.com/tree-sitter/tree-sitter-cpp/tree/70aed2e9e83eb7320ab7c454d3084300bf587037)
- [css](https://github.com/tree-sitter/tree-sitter-css/tree/769203d0f9abe1a9a691ac2b9fe4bb4397a73c51)
- [go](https://github.com/tree-sitter/tree-sitter-go/tree/64457ea6b73ef5422ed1687178d4545c3e91334a)
- [html](https://github.com/tree-sitter/tree-sitter-html/tree/86c253e675e7fdd1c0482efe0706f24bafbc3a7d)
- [java](https://github.com/tree-sitter/tree-sitter-java/tree/c194ee5e6ede5f26cf4799feead4a8f165dcf14d)
- [javascript](https://github.com/tree-sitter/tree-sitter-javascript/tree/5720b249490b3c17245ba772f6be4a43edb4e3b7)
- [json](https://github.com/tree-sitter/tree-sitter-json/tree/40a81c01a40ac48744e0c8ccabbaba1920441199)
- [python](https://github.com/tree-sitter/tree-sitter-python/tree/62827156d01c74dc1538266344e788da74536b8a)
- [rust](https://github.com/tree-sitter/tree-sitter-rust/tree/0a70e15da977489d954c219af9b50b8a722630ee)
- [tsx](https://github.com/tree-sitter/tree-sitter-typescript/tree/286e90c32060032225f636a573d0e999f7766c97)
- [typescript](https://github.com/tree-sitter/tree-sitter-typescript/tree/286e90c32060032225f636a573d0e999f7766c97)

</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>most</code></span> feature</summary>

- [asm](https://github.com/rush-rs/tree-sitter-asm/tree/36dc26acc7818920de2e103e20a9f42358caf926)
- [c_sharp](https://github.com/tree-sitter/tree-sitter-c-sharp/tree/1648e21b4f087963abf0101ee5221bb413107b07)
- [haskell](https://github.com/tree-sitter/tree-sitter-haskell/tree/ba0bfb0e5d8e9e31c160d287878c6f26add3ec08)
- [jsdoc](https://github.com/tree-sitter/tree-sitter-jsdoc/tree/189a6a4829beb9cdbe837260653b4a3dfb0cc3db)
- [php](https://github.com/tree-sitter/tree-sitter-php/tree/d38adb26304d9b9d38e9a3b4aae0ec4b29bf9462)
- [regex](https://github.com/tree-sitter/tree-sitter-regex/tree/e1cfca3c79896ff79842f057ea13e529b66af636)
- [ruby](https://github.com/tree-sitter/tree-sitter-ruby/tree/f257f3f57833d584050336921773738a3fd8ca22)
- [scala](https://github.com/tree-sitter/tree-sitter-scala/tree/8062487fb3b7f3ce1bb7ce1fd1c84bed60c75203)

</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>all</code></span> feature</summary>

- [ejs](https://github.com/tree-sitter/tree-sitter-embedded-template/tree/203f7bd3c1bbfbd98fc19add4b8fcb213c059205)
- [erb](https://github.com/tree-sitter/tree-sitter-embedded-template/tree/203f7bd3c1bbfbd98fc19add4b8fcb213c059205)
- [hexdump](https://github.com/rush-rs/tree-sitter-hexdump/tree/09eaf4fcfed00be93928d7d3d82b490cd1343b80)
- [ocaml](https://github.com/tree-sitter/tree-sitter-ocaml/tree/82e103cee0ffb61ee59f9b654b8e1d4b8e9cab74)
- [ocaml_interface](https://github.com/tree-sitter/tree-sitter-ocaml/tree/82e103cee0ffb61ee59f9b654b8e1d4b8e9cab74)
- [ql](https://github.com/tree-sitter/tree-sitter-ql/tree/bd087020f0d8c183080ca615d38de0ec827aeeaf)
- [rush](https://github.com/rush-rs/tree-sitter-rush/tree/6081a422d4f5dfb0426c09582e82e7070bb749d1)
- [verilog](https://github.com/tree-sitter/tree-sitter-verilog/tree/4457145e795b363f072463e697dfe2f6973c9a52)
- [wat](https://github.com/wasm-lsp/tree-sitter-wasm/tree/2ca28a9f9d709847bf7a3de0942a84e912f59088)

</details>

<!-- dprint-ignore-end -->
