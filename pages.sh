#!/bin/sh

set -eux
# set -o pipefail

# Script to help publish a new build to a "pages" branch for https://inland.martials.org/

# Confirm no uncommitted changes exist:
# TODO: this fails to error in the case where there are new files.
git update-index --refresh
git diff-index --quiet HEAD --

# Require main branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
    echo "run this on the main branch"
    exit 1
fi

# Ensure tests pass
cargo test

# Build for web
dx bundle --package roll --release

# Get the BUILD_DIR and check it exists
BUILD_DIR=./target/dx/roll/release/web/public
if [ ! -d "$BUILD_DIR" ]; then
    echo "build output missing: $BUILD_DIR" >&2
    exit 1
fi

# Copy build to location expected by github pages:
cp -r "$BUILD_DIR"/* ./docs

# Populate license.html
cargo about generate about.hbs > docs/license.html

# Regenerate the pages branch from the current one
git branch -D pages
git checkout -b pages

git add -f ./docs

git commit -m "Web build for pages"
git push -f --set-upstream origin pages

git checkout "$BRANCH"

echo "Done Publishing"
