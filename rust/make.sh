#! /usr/bin/bash

rm video.mp4
cargo run | ffmpeg -framerate 60 -i - video.mp4
