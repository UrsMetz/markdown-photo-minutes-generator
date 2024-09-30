# Markdown photo minutes generator

This program helps in creating markdown photo minutes (e.g. for open spaces).
It offers the following processing steps:

1. Take a directory structure that contains images of sessions
   of the following form (names of the directories and file don't matter)
   ```
   - session 1
     - image_1.jpg
     - image_2.jpg
   - session 2
     - image_1.jpg
   ```
2. Convert images as follows:
    * Create thumbnails for each image
    * Create the enlarged image version
3. Save the converted images in the given output directory
4. Create the Markdown document that includes the created thumbnails
   and links to the enlarged version of the images.
   The Markdown document is printed to `stdout`.

## How to run the program

The program has four mandatory arguments:

* the thumbnail ratio (passed as `--thumbnail-ratio <float>`)
* the directory that contains the `INPUT` structure
* the directory where the `OUTPUT` should be written to
* the `BASE_ONLINE_PATH` where the created images will be hosted

So an example invocation would be

```shell
cargo run -- --thumbnail-ratio 0.3 /path/to/input-files /path/to/output-files http://localhost/where-created-images-are-hosted
```

and would do the following:

1. go through the directory structure in `/path/to/input-files`
2. created the images
3. save them in under `/path/to/output-files`
4. print the Markdown document with the links pointing at
   `http://localhost/where-created-images-are-hosted/...` to `stdout`