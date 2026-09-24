#!/bin/bash

rm new-harness
cargo build
cp target/debug/new-harness ./
