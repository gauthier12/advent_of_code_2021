BEGIN{last=99999; num=0} $1>last {num=num+1} {last=$1} END{print num}
