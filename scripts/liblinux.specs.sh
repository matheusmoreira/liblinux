library_directory=$1
shift
startfiles=$*

cat <<specs
*link_libgcc:
+ -L$library_directory

*lib:
--start-group -llinux --end-group

*startfile:
$startfiles

*endfile:


specs
