function ojd() {
    local url="$1"
    local problem="$(echo "$url" | awk -F'/' '{print $NF}' | rev | sed 's/_/\//' | rev)"
    mkdir -p "$problem" && cd "$problem" && oj download "$url"
    cd -
    cp template/Makefile "$problem"
    cp template/main.cpp "$problem"
    cp template/compile_flags.txt "$problem"
    cd "$problem"
}

funption ojt() {
    make debug-gcc && oj t -c "./debug-gcc"
}

funption ojto() {
    make $1 && oj t -c "./$1
}
