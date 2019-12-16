cargo rustc --bin record_sort --release -- -C target-cpu=native
cp target/release/record_sort.exe record_sort.exe