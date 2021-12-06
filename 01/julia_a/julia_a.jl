using ArgParse
function parse_commandline()
   s = ArgParseSettings()
   @add_arg_table s begin
      "filename"
      help = "input filename"
      required = true
   end
   return parse_args(s)
end
function main()
   parsed_args = parse_commandline()
   open(parsed_args["filename"]) do f
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
end
main()
