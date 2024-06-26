# Rust wrapper for Jpegli library based on mozjpeg-rust

This library adds a safe(r) interface on top of jpegli for reading and writing well-compressed JPEG images.

The interface is still being developed, so it has rough edges and may change.

In particular, error handling is weird due to libjpeg's peculiar design. Error handling can't use `Result`, but needs to depend on Rust's `resume_unwind` (a panic, basically) to signal any errors in libjpeg. It's necessary to wrap all uses of this library in `catch_unwind`.

In crates compiled with `panic=abort` setting, any JPEG error will abort the process.

## Decoding example

```rust
std::panic::catch_unwind(|| -> std::io::Result<Vec<rgb::RGB8>> {
    let d = jpegli::Decompress::with_markers(jpegli::ALL_MARKERS)
        .from_path("tests/test.jpg")?;

    d.width(); // FYI
    d.height();
    d.color_space() == jpegli::ColorSpace::JCS_YCbCr;
    for marker in d.markers() { /* read metadata or color profiles */ }

    // rgb() enables conversion
    let mut image = d.rgb()?;
    image.width();
    image.height();
    image.color_space() == jpegli::ColorSpace::JCS_RGB;

    let pixels = image.read_scanlines()?;
    image.finish()?;
    Ok(pixels)
});
```

## Encoding example

```rust
# let width = 8; let height = 8;
std::panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
    let mut comp = jpegli::Compress::new(jpegli::ColorSpace::JCS_RGB);

    comp.set_size(width, height);
    let mut comp = comp.start_compress(Vec::new())?; // any io::Write will work

    // replace with your image data
    let pixels = vec![0u8; width * height * 3];
    comp.write_scanlines(&pixels[..])?;

    let writer = comp.finish()?;
    Ok(writer)
});
```


## Thanks
Huge thanks for the original authors of the [mozjpeg-rust](https://github.com/ImageOptim/mozjpeg-rust) library, which this library is based on.

Most of the code is taken from there, with some modifications to work with jpegli instead of mozjpeg.
