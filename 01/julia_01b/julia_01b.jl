open("./input") do f
   n_inc= 0
   prec_val0= 9999
   prec_val1= 9999
   prec_val2= 9999
   for (i, line) in enumerate(eachline(f))
      new_val = parse(Int32, line)
      if new_val > prec_val2
          n_inc += 1;
      end
      prec_val2 = prec_val1;
      prec_val1 = prec_val0;
      prec_val0 = new_val;
   end
   print("Total : $n_inc\n")
end
