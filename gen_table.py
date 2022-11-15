
# read in file
file = open('src/decoder.rs')
content = file.read()
file.close()

startmark = "// GEN TABLE HERE"
endmark = "// END"

# find place to insert table
contentstart, rest = content.split(startmark, maxsplit=1)

_, contentend = rest.split(endmark, maxsplit=1)

arrayname = "data"

lines = []
for i in range(0, 0b11_11_11 + 1):
    s = f"{i} =>"
    s += '{\n'

    # n \in {0, 1, 2, 3}
    n = [i & 0b11,
        i>>2 & 0b11,
        i>>4 & 0b11]


    byteoffset = 0
    # go over every single number that needs to be decoded
    for index in range(3):
        count = n[index]
        vname = "v" + str(index)

        mynum = f"let {vname} = (data[{byteoffset}] as u32)"
        for byteindex in range(1, count+1):
            mynum += f"| (data[{byteoffset + byteindex}] as u32)<< {byteindex*8}"


        mynum += ";\n"
        byteoffset += count + 1

        s += mynum

    s += "(v0, v1, v2, " + str(byteoffset) + ")},"


    lines.append(s)



newcontent = contentstart + startmark + "\n"+  "\n".join(lines) + endmark + contentend
print(newcontent)
