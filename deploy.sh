#!/bin/bash

IP=10.1.1.16

PROJECT_NAME=$(basename $(pwd))
rsync -vrlht --exclude=target/ --exclude=.git/ . pi@$IP:/home/pi/$PROJECT_NAME/
ssh pi@$IP "cd $PROJECT_NAME; and cargo build"
