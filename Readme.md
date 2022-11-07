## Idea

### Crawler

when requesting any link containing videos, a `<script>` element with the ID of `SIGI_STATE` can be found in which JSON-text containing all video-ids on the page is found. Therefore with the regex

```reg
/<script [^>]*SIGI_STATE[^>]*>(?P<json>[^<]*)<\/script>/gm
```

we can filter out this JSON.

Furthermore, the JSON contains an object called `ItemModule` containing a list of 10 videos with their ID, download link and some further information. What's important for us is the ID, the URL, a description and the author of the video.

### Uploader

Mainly there are three options on how to upload the video

1. Upload the video via the official API
2. Use a headless chrome-instance to upload the video automatically
3. reverse-engineer the API used on the front-end

Here we will discuss the advantages and disadvantages of the three options.

#### Uploading the video via the official API

Using the official API is probably the most reliable and easy way to implement the feature. Nevertheless, the API is limited to 6 video-uploads per day on the free-tier. This option would take a lot of time to start off, but as soon as we have some amount of videos uploaded, it won't be as big of a problem anymore.
Gaining access to the API might be some kind of trouble since the application we build must be registered and the authorization is quite complex.

**+**

- easy implement

**-**

- authorization
- 6 vids per day

#### Using a headless chrome-instance

The headless chrome-instance is quite straight-forward to implement. All user interactions need to be imitated by the software.
When choosing a file to upload, the headless chrome comes to its limit since there is no way of automating this process.

**+**

- unlimited videos per day

**-**

- not possible at the moment

#### reverse engineer the API

By looking at the code of the studio website we could reverse engineer the way the upload mechanism works and try to recreate the requests sent to the server.

**+**

- unlimited videos per day

**-**

- time consuming
- unstable
