#!/bin/bash
trunk build --release
cd dist
git init
git add -A
git commit -m "Deploy"
git push -f git@github.com:VishnuKMi/blockmaxi.git main:gh-pages
