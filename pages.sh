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

# Build for web
dx bundle --package roll --release

# Regenerate the pages branch from the current one
git branch -d pages
git checkout -b pages

# Copy build to location expected by github pages:
cp -r ./target/dx/roll/release/web/public/* ./docs

cargo about generate about.hbs > docs/license.html

git add -f ./docs

git commit -m "Web build for pages"
git push -f --set-upstream origin pages

git checkout $BRANCH

echo "Done Publishing"