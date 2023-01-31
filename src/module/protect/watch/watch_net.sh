cat /proc/net/dev | grep -E 'ens|eth' | awk '{print $2, $10}';
# cat /proc/net/dev | grep -E 'ens|eth' | awk '{print $10}';
sleep 0.5;
cat /proc/net/dev | grep -E 'ens|eth' | awk '{print $2, $10}'
# cat /proc/net/dev | grep -E 'ens|eth' | awk '{print $10}';