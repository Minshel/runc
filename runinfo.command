[release]
echo yura daun

[cargo-build]
cargo clean;
cargo build;
powershell -Command "cp .\runinfo.command .\target\debug\runinfo.command";
