# Typo-eq

Typo-eq is a typing training app for other languages. All it needs is a 
dictionary for words and their translations.

Currently this app **only supports xdxf** dictionary files, so be sure to 
download one for the language you want. I recomend the Swedish-English 
dictinary available at https://folkets-lexikon.csc.kth.se/folkets/om.en.html.

To start the application, make sure you have `rust` installed and put your
dictionary data somewhere accessible to the app. Then run the following
command in your terminal.

```sh
cargo run -- --dict path/to/xdxf/file
```

An application window should start up with a short blank screen while the
dictionary is being loaded. Then a word will appear and you can begin typing.