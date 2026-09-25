cargo run -- main.nml > src.c
python delete.py
gcc src.c -o src.exe
./src.exe