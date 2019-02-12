#!/bin/bash
cp $1 "$1.copy" && cat $1 | grep \#[0-9A-F]*\< | sed -r 's/<\/?string>//g' | sed -r 's/\s?//g' | sed -r 's/(.+)/\1 yes/g' | xargs -L1 target/release/monochromize | xargs -L1 -t -i sed -i.bak {} "$1.copy"
