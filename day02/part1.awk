{
    v = 0
    for (i=3; i<=NF; i++) {
        sub(/[,;]/, "", $i)
        if (($i == "red" && v > 12) ||
            ($i == "green" && v > 13) ||
            ($i == "blue" && v > 14)) {
                next
        }
        v = $i
    }
    t += NR
}

END {
    print t
}
