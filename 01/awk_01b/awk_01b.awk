BEGIN{l1=99999;l2=99999;l3=99999; num=0} $1>l3 {num=num+1} {l3=l2;l2=l1;l1=$1} END{print num}
