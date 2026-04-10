cargo rustc -- -C link-arg=-undefined -C link-arg=-dynamic_lookup
ln -s ../target/debug/libgearley_lua.so ./gearley_lua.so
lua5.4 lua-test/test.lua