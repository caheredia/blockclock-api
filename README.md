# Resources
Ureq examples
- https://github.com/algesten/ureq
- https://crates.io/crates/ureq/2.5.0

Parsing toml files, tests included
- https://docs.rs/toml/latest/toml/
- https://github.com/toml-rs/toml/blob/main/crates/toml/examples/decode.rs

Blockclock mini push API
- https://blockclockmini.com/api

Python Examples
- https://www.notion.so/BLOCKCLOCKmini-API-05f6c24cf71748098094508ca5b2dd7a

# Example API calls
```console
❯ curl --digest "http://192.168.10.196/api/status"  

{"tags": ["cm.bitnodes.reachable_bitcoin_nodes", "datetime.local.date", "cm.markets.sats_per_dollar"], "is_micro": false, "showing": "datetime.local.date", "version": "1.2.2", "rendered": {"number": null, "pair": ["DATE", null], "tl_text": "Local Date", "label": "Local Date", "contents":["D", "e", "c", " ", "2", "6", "/DATE/2022"], "string": null, "br_text": null, "is_error": false, "tag": "datetime.local.date"}, "menu_active":false}
```

```console
❯ curl --digest "http://192.168.10.196/api/status"

{"tags": ["cm.bitnodes.reachable_bitcoin_nodes", "datetime.local.date", "cm.markets.sats_per_dollar"], "is_micro": false, "showing": "cm.markets.sats_per_dollar", "version": "1.2.2", "rendered": {"number": 5925.0, "pair": ["SATS", "1USD"], "tl_text": "Value of one US Dollar, expressed in Satoshis", "label": "Value of one US Dollar, expressed in Satoshis", "contents": ["/SATS/1USD", " ", " ", "5", "9", "2", "5"], "string": null, "br_text": null, "is_error": false, "tag": "cm.markets.sats_per_dollar"}, "menu_active": false}

```
Example with `Hi You` does not work. Spaces are not rendered correctly.
```console
❯ curl --digest "http://192.168.10.196/api/status"

{"tags": ["cm.bitnodes.reachable_bitcoin_nodes", "datetime.local.date", "cm.markets.sats_per_dollar"], "is_micro": false, "showing": "static.api", "version": "1.2.2", "rendered": {"number": null, "pair": null, "tl_text": null, "label": null, "contents": ["i", "%", "2", "0", "Y", "o", "u"], "string": null, "br_text": null, "is_error": false, "tag": "static.api"}, "menu_active": false}
```
