# ginkou-api

A REST api for interacting with [ginkou](https://github.com/cronokirby/ginkou).
Find examples of sentences that use a specific Japanese word regardless of conjugation.

## Usage

https://japanese.hifumi.io/lookup/僕

## Dependencies

This program depends on [mecab](http://taku910.github.io/mecab/) for the aforementioned
morphological splitting. For instructions on installing it, see the [mecab crate](https://github.com/tsurai/mecab-rs).

You can use the following packages if you use `apt`

```
sudo apt install libsqlite3-dev
sudo apt install libmecab-dev
```

For a CLI interface, download the original [ginkou](https://github.com/cronokirby/ginkou) app.
