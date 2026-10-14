# Dailymotion trial

Test URL: https://www.dailymotion.com/video/xbgpel6

The repository builds with Rust 1.97.1. With yt-dlp 2026.08.19, the Rust
example currently reproduces a metadata parsing bug: `missing field preference`.
Dailymotion thumbnails omit `preference`; its video metadata also omits
`live_status` and `playable_in_embed`. The library requires those fields.
Track the parser fix in https://github.com/chusri/yt-dlp/issues/2.

## Run the Rust reproduction

Install Rust, yt-dlp, and FFmpeg, and make the two binaries available on PATH.

```sh
cargo run --example dailymotion -- \
  https://www.dailymotion.com/video/xbgpel6 dailymotion-sample.mp4 30
```

The example uses this library to extract metadata and FFmpeg to assemble a
short HLS sample. It will stop at metadata extraction until issue #2 is fixed.
It refuses to overwrite an existing output file.

## Working tool-only trial

This bypasses the Rust parser and was verified to save a playable sample:

```sh
yt-dlp --no-playlist --download-sections '*0-30' \
  -f hls-380 -o dailymotion-sample.mp4 \
  https://www.dailymotion.com/video/xbgpel6
```

The result contains H.264 video at 360 x 640 and AAC audio, runs 30.155 seconds,
and is 1,765,818 bytes. Stream copying can extend the clip slightly past the
requested endpoint. The full source video is 5,025 seconds.

To download the full video with the working tool, omit `--download-sections`:

```sh
yt-dlp --no-playlist -f hls-380 -o dailymotion-full.mp4 \
  https://www.dailymotion.com/video/xbgpel6
```

Full-video downloading has not been tested. These format IDs are specific to
this video; use `yt-dlp -F URL` when trying another video.
