require 'set'

### PART 1 ###

moves = {
    "D" => [0, -1],
    "U" => [0, 1],
    "L" => [-1, 0],
    "R" => [1, 0]
}

def add_pos(a,b)
    [a[0]+b[0], a[1]+b[1]]
end

def neighbor(a,b)
    (a[0] - b[0]).abs <= 1 && (a[1] - b[1]).abs <= 1
end

# Calculate what direction b needs to move to be neighbor to a
# Returns [0,0] if no move is necessary
def calc_move(a,b)
    return [0,0] if neighbor(a,b)
    x = a[0] - b[0]
    y = a[1] - b[1]
    if x.abs > 1
        x = x/2 # -2 becomes -1 and 2 becomes 1
    end
    if y.abs > 1
        y = y/2
    end
    return [x, y]
end

pos_h = [0,0]
pos_t = [0,0]
visited = Set::new([pos_t])

File.open("input.txt").each do |line|
    dir = line.split(' ')[0]
    count = line.split(' ')[1].to_i

    count.times do
        pos_h = add_pos(pos_h,moves[dir])
        pos_t = add_pos(pos_t, calc_move(pos_h, pos_t))
        visited.add(pos_t)
    end
end

puts visited.count

### PART 2 ###
# knots[0] is the head followed by the "tails"
knots = [[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]]
visited = Set::new([knots[9]])

File.open("input.txt").each do |line|
    dir = line.split(' ')[0]
    count = line.split(' ')[1].to_i

    count.times do
        (0..9).each do |i|
            if i == 0
                knots[i] = add_pos(knots[i],moves[dir])
            else
                move = calc_move(knots[i-1], knots[i])
                knots[i] = add_pos(knots[i], calc_move(knots[i-1], knots[i]))
            end
            visited.add(knots[i]) if i == 9
        end
    end
end

puts visited.count