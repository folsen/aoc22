### PART 1 ###
pairs = []
File.open("input.txt").each do |line|
    pair = line.strip.split(',')
    elf1_range = pair[0].split('-').map(&:to_i)
    elf2_range = pair[1].split('-').map(&:to_i)
    pairs.push({
        elf1: (elf1_range[0]..elf1_range[1]).to_a,
        elf2: (elf2_range[0]..elf2_range[1]).to_a
    })
end

def is_fully_overlapping(pair)
    # The length of the union doesn't increase the over the size of the largest list
    [pair[:elf1].length, pair[:elf2].length].max == (pair[:elf1] | pair[:elf2]).length ? 1 : 0
end

puts pairs.map{|pair| is_fully_overlapping(pair)}.sum

### PART 2 ###

def is_overlapping(pair)
    # The sum of the length of both lists is greater than the length of the union
    [pair[:elf1].length, pair[:elf2].length].sum > (pair[:elf1] | pair[:elf2]).length ? 1 : 0
end

puts pairs.map{|pair| is_overlapping(pair)}.sum