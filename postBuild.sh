otool -L target/debug/clang-rs-test
# echo ""
# echo "change libclang rpath"
# echo ""
# install_name_tool -change @rpath/libclang.dylib /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libclang.dylib target/debug/clang-rs-test
#otool -L target/debug/clang-rs-test
