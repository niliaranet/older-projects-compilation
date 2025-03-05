echo "use sudo? [y/n]"
read answer

if [[ $answer == 'y' ]] then
    sudo="sudo"
fi

$sudo mariadb -u root < ./scripts/fast_populate.sql
