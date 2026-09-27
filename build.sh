#!/bin/bash

rm sirk
cargo build
cp target/debug/sirk ./
