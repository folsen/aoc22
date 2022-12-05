### PART 1 ###

row_stacks = []
instructions = []
File.open("input.txt").each_with_index do |line, i|
    if i<8
        row_stack = line.chars.each_slice(4).to_a.map{|cs| cs[1] == ' ' ? nil : cs[1]}
        row_stacks.push(row_stack) 
    elsif i>=10
        digits = line.match(/move (\d+) from (\d+) to (\d+)/)
        instructions.push([digits[1].to_i, digits[2].to_i - 1, digits[3].to_i - 1])
    end
end

stacks_part1 = row_stacks.transpose.map(&:compact).map(&:reverse)
stacks_part2 = stacks_part1.map(&:clone)

# Mutates the stacks per instructions

instructions.each do |instruction|
    instruction[0].times do
        stacks_part1[instruction[2]].push(stacks_part1[instruction[1]].pop)
    end
end

puts stacks_part1.map(&:pop).join

### PART 2 ###

instructions.each do |instruction|
    nmbr = instruction[0]
    from_stack = stacks_part2[instruction[1]]
    to_stack = stacks_part2[instruction[2]]
    slice = from_stack.slice!(from_stack.length-nmbr,from_stack.length)
    to_stack.concat(slice)
end

puts stacks_part2.map(&:pop).join