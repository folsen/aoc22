require 'prime'

# Format is old = old [op] [val]
def parse_operation(equation)
    op = equation[3]
    val = equation[4]
    if op == "+" && val == "old"
        lambda {|old| old + old}
    elsif op == "*" && val == "old"
        lambda {|old| old * old}
    elsif op == "+"
        lambda {|old| old + val.to_i}
    elsif op == "*"
        lambda {|old| old * val.to_i}
    end
end

def read_input()
    # Need to parse a monkey over several lines and then reset it when a new monkey
    # line comes
    monkeys = []
    monkey = {}
    File.open("input.txt").each do |line|
        words = line.split(' ')
        case words[0]
        when "Monkey"
            monkeys.push(monkey) if words[1].to_i > 0
            monkey = {}
            monkey[:number] = words[1].to_i
        when "Starting"
        monkey[:items] = words.drop(2).map(&:to_i) 
        when "Operation:"
            monkey[:operation] = parse_operation(words.drop(1))
        when "Test:"
            monkey[:test] = words.last.to_i
        else
            if words[1] == "true:"
                monkey[:test_true] = words.last.to_i
            elsif words[1] == "false:"
                monkey[:test_false] = words.last.to_i
            end
        end
    end
    monkeys.push(monkey)
    return monkeys
end

monkeys = read_input()

# Run the game for 20 rounds
inspections = [] # An array of inspections by monkey index
20.times do
    monkeys.each do |monkey|
        monkey[:items].each do |item|
            item = monkey[:operation].call(item)
            item = item / 3
            inspections[monkey[:number]] = inspections[monkey[:number]].to_i + 1
            if item % monkey[:test] == 0
                monkeys[monkey[:test_true]][:items].push(item)
            else
                monkeys[monkey[:test_false]][:items].push(item)
            end
        end
        monkey[:items] = [] # This monkey has passed along all its items now
    end
end

puts inspections.sort[-2..].inject(:*)

monkeys = read_input()
# Run the game for 10000 rounds and don't divide by 3
inspections = [] # An array of inspections by monkey index
# Had to cheat to get this line. Explanation can be found here:
# https://aoc.just2good.co.uk/2022/11
# "modulo congruence is preserved for any multiplication or addition operations"
modulus = monkeys.map{|m| m[:test]}.reduce(1,:*)
10000.times do
    monkeys.each do |monkey|
        monkey[:items].each do |item|
            item = monkey[:operation].call(item)
            inspections[monkey[:number]] = inspections[monkey[:number]].to_i + 1
            item = item % modulus
            t = item % monkey[:test] == 0
            if t
                monkeys[monkey[:test_true]][:items].push(item)
            else
                monkeys[monkey[:test_false]][:items].push(item)
            end
        end
        monkey[:items] = []
    end
end

puts inspections.sort[-2..].inject(:*)