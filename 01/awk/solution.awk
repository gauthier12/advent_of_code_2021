#solution A
awk 'BEGIN{last=99999; num=0} $1>last {num=num+1} {last=$1} END{print num}' input
#solution  B
awk 'BEGIN{l1=99999;l2=99999;l3=99999; num=0} $1+l1+l2>l1+l2+l3 {num=num+1} {l3=l2;l2=l1;l1=$1} END{print num}' input
