#!/bin/bash

list_dirs=$( ls --directory */ )

echo "Cleaning ..."
for d in $list_dirs; do
    echo "$d"
    cd "$d"
    if [ -f "Cargo.toml" ]; then
        cargo clean
    fi
    cd ..
done

exit 0
