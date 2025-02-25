#!/bin/bash

./commands.sh app/container/build &
./commands.sh db/container/build &
wait

./commands.sh fc/app/migrate/up-all
./commands.sh fc/app/wasm-build

echo
echo "All set!"
echo "Next step : ./commands.sh fc/app/server-start"
echo "Happy coding! :D"