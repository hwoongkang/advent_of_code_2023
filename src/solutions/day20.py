with open("./dot.txt", "w") as f_out:
    with open("./input.txt", "r") as f_in:
        f_out.write("digraph {\n")
        for line in f_in:
            l = line
            if line.startswith("%") or line.startswith("&"):
                l = line[1:]
            l = l.split("->")
            name = l[0].strip()
            output = map(lambda x: x.strip(), l[1].split(","))
            
            for o in output:
                f_out.write(f"{name} -> {o}\n")
        f_out.write("}")

