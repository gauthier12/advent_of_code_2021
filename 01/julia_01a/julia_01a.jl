open("./input") do f
   n_inc= 0
   prec_val= 9999
   for (i, line) in enumerate(eachline(f))
      new_val = parse(Int32, line)
      if new_val > prec_val
          n_inc += 1;
      end
      prec_val = new_val;
   end
   print("Total : $n_inc\n")
end
