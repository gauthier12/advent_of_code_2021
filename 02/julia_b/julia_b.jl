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
      pos_hor = 0
      pos_ver = 0
      aim = 0
      for (i, line) in enumerate(eachline(f))
         command, val = split(line, " ")
         new_val = parse(Int32, val)
         if command == "up"
            aim -= new_val
         elseif command == "down"
            aim += new_val
         elseif command == "forward"
            pos_hor += new_val
            pos_ver += aim * new_val
         end
      end
      println("position depth ", pos_ver, " hor ", pos_hor);
      println("solution ", pos_ver * pos_hor);
   end
end
main()
