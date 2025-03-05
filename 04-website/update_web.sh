#!/bin/bash
hugo build
rsync -avrP --delete-after ./public/ user@niliara.net:/var/www/niliara.net
