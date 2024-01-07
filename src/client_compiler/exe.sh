# !usr/bin/env sh

# Params 
# build command, run command, working directory

buildCmd=$1
runCmd=$2
wd=$3

if [ "$wd" == "" ]; then 
    # cd client
    echo h
else 
    cd $wd || (echo "CD failed"; exit)
fi

if [ "$buildCmd" == "" ]; then
    echo "Build Command is not valid"
    exit
fi 

TIMEFORMAT="%R"

echo "Starting Project build: $buildCmd"
start=$(date +%s%N)
eval $buildCmd
end=$(date +%s%N)

echo "Res: $(($end-$start)))"
echo "Starting Client Server"
eval $runCmd