#!/bin/bash

set -o errexit

shopt -s globstar

cargo doc --no-deps

REPO=`git config remote.origin.url`
SSH_REPO=${REPO/https:\/\/github.com\//git@github.com:}
SHA=`git rev-parse --verify HEAD`

git clone --branch gh-pages $REPO deploy_docs
cd deploy_docs

git config user.name "Julius de Bruijn"
git config user.email "julius.debruijn@360dialog.com"

if [ "$TRAVIS_TAG" = "" ]; then
    rm -rf master
    mv ../target/doc ./master
    echo "<meta http-equiv=refresh content=0;url=a2/index.html>" > ./master/index.html
else
    rm -rf $TRAVIS_TAG
    mv ../target/doc ./$TRAVIS_TAG
    echo "<meta http-equiv=refresh content=0;url=a2/index.html>" > ./$TRAVIS_TAG/index.html

    latest=$(echo * | tr " " "\n" | sort -V -r | head -n1)
    if [ "$TRAVIS_TAG" = "$latest" ]; then
        echo "<meta http-equiv=refresh content=0;url=$latest/a2/index.html>" > index.html
    fi
fi

git add -A .
git commit -m "rebuild pages at ${TRAVIS_COMMIT}"

ENCRYPTED_KEY_VAR="encrypted_${ENCRYPTION_LABEL}_key"
ENCRYPTED_IV_VAR="encrypted_${ENCRYPTION_LABEL}_iv"
ENCRYPTED_KEY=${!ENCRYPTED_KEY_VAR}
ENCRYPTED_IV=${!ENCRYPTED_IV_VAR}

openssl aes-256-cbc -K $ENCRYPTED_KEY -iv $ENCRYPTED_IV -in ../a2_travis.enc -out a2_travis -d
chmod 600 a2_travis
eval `ssh-agent -s`
ssh-add a2_travis

echo
echo "Pushing docs..."
git push $SSH_REPO gh-pages
echo
echo "Docs published."
echo
