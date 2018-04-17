#!/usr/bin/env bash

if [ "$#" -ne 1 ]; then
    echo "Usage: create-app <name> <bundle id>"
fi

cp -r Template.app ./template
sed -i '' 's/TEMPLATE_NAME/$1/g' ./template/Info.plist
sed -i '' 's/TEMPLATE_ID/$2/g' ./template/Info.plist
plutil -convert binary1 ./template/Info.plist
