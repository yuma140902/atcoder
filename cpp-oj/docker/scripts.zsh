function ojd() {
    local url="$1"
    local problem="$(echo "$url" | awk -F'/' '{print $NF}' | rev | sed 's/_/\//' | rev)"
    mkdir -p "$problem" && cd "$problem" && oj download "$url"
}

function ojt() {
    local prog="$0"
    local compiler="gcc"
    local mode="debug"
    while [[ $1 -gt 0 ]]; do
        case $1 in
            -h|--help)
                echo "$prog [options]"
                echo ""
                echo "OPTIONS:"
                echo "  -h, --help       show this message"
                echo "  -c, --compiler   'gcc' or 'clang' [default=gcc]"
                echo "  -d, --debug      debug mode [default]"
                echo "  -o, --optimized  optimized mode"
                exit 0
                ;;
            -c|--compiler)
                local compiler="$2"
                shift
                shift
                ;;
            -d|--debug)
                local mode=debug
                shift
                ;;
            -o|--optimized)
                local mode=optimized
                shift
                ;;
            *)
                echo "Unknown option $1"
                exit 1
                ;;
        esac
    done
    make $mode-$compiler && oj t -c "./$mode-$compiler"
}
