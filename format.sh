fd --type f --extension rs "" | pv -N "Processing Rust files" -l >/dev/null
fd --type f --extension rs "" -x sed -i 's/    /\t/g'
