{
    r = 0
    g = 0
    b = 0

    v = 0
    for (i=3; i<=NF; i++) {
        sub(/[,;]/, "", $i)
        if ($i == "red") {
            r = v > r ? v : r
        } else if ($i == "green") {
            g = v > g ? v : g
        } else if ($i == "blue") {
            b = v > b ? v : b
        } else {
            v = $i
        }
    }
    t += r * g * b
}

END {
    print t
}

