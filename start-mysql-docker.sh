#!/bin/bash

docker run --rm --name mysql -e MYSQL_ROOT_PASSWORD=docker -p 53306:3306 -v /tmp/mysql:/var/lib/mysql -d mysql:8.0.22
