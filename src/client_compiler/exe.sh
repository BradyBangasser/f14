# !/usr/bin/env sh

# framework cli cmd, subcmd, dev mode, start server

echo "Starting client build" 

frameworkCli=$1
subcmd=$2
devMode=$3
startServer=$4

# if devMode != false
# set devMode to true
if [ "$devMode" != "false" ]; then 
    devMode="true"
fi

# if startServer != false
# set startServer to true
if [ "$startServer" != "false" ]; then 
    startServer="true"
fi 

# fix this
# if frameworkCli == false
if [ "$frameworkCli" == "false" ]; then
    # attempt to create router for the src dir

    if node src/client_compiler/createClientRouter.js; then
        echo if
    else
        echo else
    fi
fi

# if subcmd == ""
# attempt to run, but add extra warning message if it fails
if [ "$subcmd" == "" ]; then
    echo here
fi

# Attempt to exec the build script 
if eval ""; then
    echo success
else 
    echo "fail"
fi

echo $frameworkCli this