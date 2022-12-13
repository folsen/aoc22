require 'json'

def pairs_for(input_file)
    pairs = []
    pair = []
    File.open(input_file).each do |line|
        if pair.length == 2
            # Reset the pair and skip the empty line at the same time
            pairs.push(pair)
            pair = []
        else
            pair.push(JSON.parse(line))
        end
    end
    pairs.push(pair)
    return pairs
end

def is_right_order(left, right)
    if left.class == Integer && right.class == Integer
        if left == right 
            :next
        else
            left < right
        end
    elsif left == [] && right == []
        :next
    elsif left.class == Array && right.class == Array
        res = is_right_order(left[0], right[0])
        if res == :next
            is_right_order(left.drop(1), right.drop(1))
        else
            res
        end
    elsif left.class == Array && right.class == Integer
        is_right_order(left, [right])
    elsif left.class == Integer && right.class == Array
        is_right_order([left], right)
    elsif left.nil? && !right.nil?
        true
    elsif !left.nil? && right.nil?
        false
    else
        p "warning, made it to end, #{left}, #{right}"
    end
end


### Part 1 ###
pairs = pairs_for("input.txt")
index_sum = 0
pairs.each_with_index do |pair, i|
    if is_right_order(pair[0], pair[1])
        index_sum += i+1
    end
end

p index_sum


### Part 2 ###
pairs = pairs_for("input.txt")
pairs.push([[[2]], [[6]]]).flatten!(1)
pairs.sort!{|a,b| if is_right_order(a,b) then -1 else 1 end}
first = pairs.index{|x| x == [[2]]} + 1
second = pairs.index{|x| x == [[6]]} + 1
p first * second
