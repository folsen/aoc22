### PART 1 ###

elves = []
elf = []

File.open("input.txt").each do |line|

    if line.length > 1
        elf.push(line)
    else
        elves.push(elf.map(&:to_i))
        elf = []
    end
end

most_calories = 0
elves.each do |elf|
    elf_max = elf.sum
    if elf_max > most_calories
        most_calories = elf_max
    end
end

puts most_calories

### PART 2 ###

puts elves.map(&:sum).sort_by{|a| a}.reverse.take(3).sum