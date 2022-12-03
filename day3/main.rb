### PART 1 ###

## Fill a hash with priorities
priorities = {}
('a'..'z').each_with_index do |c, i|
    priorities[c] = i + 1
end
('A'..'Z').each_with_index do |c, i|
    priorities[c] = i + 1 + 26
end


rucksacks = []
File.open("input.txt").each do |line|
    rucksacks.push(line.strip)
end

def split_compartments(contents)
    half = contents.length/2
    [contents[0..half-1], contents[half..(half*2)-1]]
end

def find_intersection(pockets)
    (pockets[0].chars & pockets[1].chars)[0]
end

split_rucksacks = rucksacks.map{ |rucksack| split_compartments(rucksack) }

split_rucksacks.map!{|rucksack| find_intersection(rucksack)}

puts split_rucksacks.map{|item| priorities[item]}.sum

### PART 2 ###

chunked_rucksacks = rucksacks.each_slice(3).to_a

chunked_rucksacks.map!{|chunk| (chunk[0].chars & chunk[1].chars & chunk[2].chars)[0]}

puts chunked_rucksacks.map!{|item| priorities[item]}.sum