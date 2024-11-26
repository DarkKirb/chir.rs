#!/usr/bin/env nix-shell
#! nix-shell -p openssh -i bash

openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout secrets/server.key -out secrets/server.crt