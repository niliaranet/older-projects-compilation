#!/bin/bash
echo "choose a password for the database:"
read -s input

echo "use sudo? [y/n]"
read answer

if [[ $answer == 'y' ]] then
    sudo="sudo"
fi

$sudo mariadb -u root <<EOF
CREATE OR REPLACE USER 'balalaika_user'@'%' IDENTIFIED BY '$input';
EOF

$sudo mariadb -u root < ./scripts/create_db.sql
