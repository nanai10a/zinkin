#!/bin/sh
openssl genpkey -algorithm ed25519 -outform DER | base64
