#!/bin/bash

# auto download the latest version if there is a new one, then launch the game

VERSION_FILE="$HOME/jungle_version"
DEPLOYED_VERSION_URL="https://brooks-builds-games.s3-us-west-1.amazonaws.com/jungle/version"
GAME_URL="https://brooks-builds-games.s3-us-west-1.amazonaws.com/jungle/jungle_game.zip"
GAME_DIRECTORY="$HOME/jungle"

if [ -d $GAME_DIRECTORY ]
then 
    cd "$GAME_DIRECTORY" || exit
else
    mkdir "$GAME_DIRECTORY"
    cd "$GAME_DIRECTORY" || exit
fi

if [ -f $VERSION_FILE ]
then
    CURRENT_VERSION=$(cat $VERSION_FILE)
else
    echo "0.0.0" > $VERSION_FILE
    CURRENT_VERSION="0.0.0"
fi
echo $CURRENT_VERSION

DEPLOYED_VERSION=$(curl $DEPLOYED_VERSION_URL)

echo current: $CURRENT_VERSION deployed: $DEPLOYED_VERSION
if [ "$CURRENT_VERSION" != "$DEPLOYED_VERSION" ]
then
    curl $GAME_URL > jungle.zip
    unzip -o jungle.zip
    echo $DEPLOYED_VERSION > $VERSION_FILE
    cp target/release/jungle ./
fi
    
./jungle