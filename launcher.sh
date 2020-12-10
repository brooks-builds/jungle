#!/bin/bash

# auto download the latest version if there is a new one, then launch the game

VERSION_FILE="$HOME/jungle_version"
DEPLOYED_VERSION_URL="https://brooks-builds-games.s3-us-west-1.amazonaws.com/jungle/version"
REPO="$HOME/jungle_repo"
TARGET_RESOURCES="$REPO/target/release/resources"

if [ -f $VERSION_FILE ]
then
    CURRENT_VERSION=$(cat $VERSION_FILE)
else
    echo "0.0.0" > $VERSION_FILE
    CURRENT_VERSION="0.0.0"
fi

DEPLOYED_VERSION=$(curl $DEPLOYED_VERSION_URL)

if [ "$CURRENT_VERSION" != "$DEPLOYED_VERSION" ]
then
    git pull
    if [ -d $TARGET_RESOURCES ]
    then
        rm -rf $TARGET_RESOURCES
    fi
    cp -r resources $TARGET_RESOURCES
    echo $DEPLOYED_VERSION > $VERSION_FILE
fi
    
cargo run --release
