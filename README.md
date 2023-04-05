# Usage:

On your test file, instead to init you own deps, call `create_http_mock` function:

`let deps: OwnedDeps<_, _, HttpWasmMockQuerier<DefaultWasmMockQuerier>> = create_http_mock(None, url_lcd, None);`

`Type annotation` is needed. If your chain doesn't have any particular custom query, you can leave specify the default as above:

`let deps: OwnedDeps<_, _, HttpWasmMockQuerier<DefaultWasmMockQuerier>>`