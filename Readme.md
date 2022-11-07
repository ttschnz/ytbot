when requesting any tiktok link containing videos, a `<script>` element with the ID of `SIGI_STATE` can be found in which JSON-text containing all video-ids on the page is found. Therefore with the regex

```reg
/<script [^>]*SIGI_STATE[^>]*>(?P<json>[^<]*)<\/script>/gm
```

we can filter out this JSON.
