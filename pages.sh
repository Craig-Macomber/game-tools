#!/bin/bash
set -eux -o pipefail

# Script to help publish a new build to a "pages" branch for https://craig-macomber.github.io/rusty-flame/

# Confirm no uncommitted changes exist:
# TODO: this fails to error in the case where there are new files.
git update-index --refresh
git diff-index --quiet HEAD --

# Require main branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" != "main" ]]; then
    echo "run this on the main branch"
    exit 1
fi

# Ensure tests pass
cargo test

# Web release build
# wasm-pack build --target web

# Build for web
dx bundle --package roll --platform web --outdir pages

# Regenerate the pages branch from the current one
git branch -d pages
git checkout -b pages

git add -f ./pages

# cp -r ./target/dx/roll/release/web/public/* .

# git add -f index.html
# git add -f assets
# git add -f wasm

git commit -m "Web build for pages"
git push -f --set-upstream origin pages

git checkout $BRANCH

echo "Done Publishing"