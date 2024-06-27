cd database
cargo build --release
cd ..
::copy "./database/target/release/lib_db.rlib" "./server/lib/lib_db.rlib"
cd server
cargo build --release
pause