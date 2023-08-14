# conceptnet-trim

trim [conceptnet](https://conceptnet.io/)'s ~34,000,000 multilingual assertions
(about 10gb of tsv) into a tidy ~3,400,000 english-language assertions (in json
format).

1. clone this repo
2. [download the latest version of
   conceptnet](https://github.com/commonsense/conceptnet5/wiki/Downloads) (5.7.0
   at the time of writing)
3. extract it to `data/assertions.csv` in the root of this repo
4. run `cargo run -r` to run in release mode. the trimmed assertions will be
   written to `data/trimmed.json`.

or download a pre-trimmed file from the [releases
page](https://github.com/lostfictions/conceptnet-trim/releases).