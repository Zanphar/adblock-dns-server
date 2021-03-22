#!/bin/sh

./replace.sh

cp config/named.conf.local /etc/bind/named.conf.local
cp config/named.conf.options /etc/bind/named.conf.options
cp config/null.zone.file /etc/bind/null.zone.file

mkdir -p /logs
touch /logs/rpz_log.txt
chmod a+rwx -R /logs

named -f -u named
