TEMP=$(mktemp -d)/fortune-mod
DATS=$TEMP/fortune-mod/datfiles
OUT=cowbot/assets/fortune/datfiles
git clone https://github.com/shlomif/fortune-mod $TEMP
# clean it up
rm -r $DATS/data $DATS/off $DATS/CMakeLists.txt
# move into assets
cp -r $DATS $OUT
