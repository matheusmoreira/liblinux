specs=$1

cat <<wrapper
#!/bin/sh
exec gcc -specs=$specs "\$@"
wrapper
