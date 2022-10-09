#!/bin/sh
EZP=target/debug/ezp
#EZP=cargo run -- 
dd if=/dev/random of=input.bin bs=1048576 count=1
$EZP erase
$EZP write --type='EN25T80' input.bin
$EZP read  --type='EN25T80' output.bin
md5 input.bin output.bin
