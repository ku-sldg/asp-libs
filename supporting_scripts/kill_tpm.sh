#!/bin/bash
read -p "This will kill all instances of swtpm and tpm2-abrmd, are you sure (this is probably ok) (y/N)? " -r
if [[ $REPLY =~ ^[Yy](([Ee][Ss])?)$ ]]
then
    killall swtpm
    killall tpm2-abrmd
fi
