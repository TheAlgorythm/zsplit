#! /bin/bash

dark_contrast='#fff'
light_contrast='#000'
sd '<defs/>' "<style>.contrastFill{fill:$light_contrast;}.contrastStroke{stroke:$light_contrast;}.contrastColor{color:$light_contrast;}@media(prefers-color-scheme:dark){.contrastFill{fill:$dark_contrast;}.contrastStroke{stroke:$dark_contrast;}.contrastColor{color:$dark_contrast;}}</style><defs/>" docs/simple.svg
sd 'stroke="#([Ff]{3}|[Ff]{6})"' 'class="contrastStroke"' docs/simple.svg
sd 'fill="#([Ff]{3}|[Ff]{6})"' 'class="contrastFill"' docs/simple.svg
sd 'style="([^#"]*)color:\s*#([Ff]{3}|[Ff]{6});\s*([^"]*)"' 'style="${1}${3}" class="contrastColor"' docs/simple.svg
sd 'class="([a-zA-Z]+)" class="([a-zA-Z]+)" class="([a-zA-Z]+)"' 'class="${1} ${2} ${3}"' docs/simple.svg
sd 'class="([a-zA-Z]+)" class="([a-zA-Z]+)"' 'class="${1} ${2}"' docs/simple.svg
