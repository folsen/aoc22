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
def execute_instructions(stacks, instruction)
    instruction[0].times do
        stacks[instruction[2]].push(stacks[instruction[1]].pop)
    end
end

instructions.each do |instruction|
    execute_instructions(stacks_part1, instruction)
end

puts stacks_part1.map(&:pop).join

### PART 2 ###

def execute_instructions_9001(stacks, instruction)
    nmbr = instruction[0]
    from_stack = stacks[instruction[1]]
    to_stack = stacks[instruction[2]]
    slice = from_stack.slice!(from_stack.length-nmbr,from_stack.length)
    to_stack.concat(slice)
end

instructions.each do |instruction|
    execute_instructions_9001(stacks_part2, instruction)
end

puts stacks_part2.map(&:pop).join