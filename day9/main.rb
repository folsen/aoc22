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

# Used for part1 but then refactored out for part2
#def neighbor(a,b)
#    (a[0] - b[0]).abs <= 1 && (a[1] - b[1]).abs <= 1
#end
#
#def reverse(dir)
#    {"D" => "U", "U" => "D", "L" => "R", "R" => "L"}[dir]
#end

# Calculate what direction b needs to move to be neighbor to a
# Returns [0,0] if no move is necessary
def calc_move(a,b)
    case [a[0] - b[0], a[1] - b[1]]
    # Needs to move up
    when [0,2] then [0,1]
    # Needs to move down
    when [0,-2] then [0,-1]
    # Needs to move left
    when [-2,0] then [-1,0]
    # Needs to move right
    when [2,0] then [1,0]
    # Needs to move diagonally upleft 
    when [-1,2] then [-1,1]
    # Needs to move diagonally upright 
    when [1,2] then [1,1]
    # Needs to move diagonally downleft 
    when [-1,-2] then [-1,-1]
    # Needs to move diagonally downright 
    when [1,-2] then [1,-1]
    # Needs to move diagonally leftdown
    when [-2,-1] then [-1, -1]
    # Needs to move diagonally rightdown
    when [2,-1] then [1, -1]
    # Needs to move diagonally leftup
    when [-2,1] then [-1, 1]
    # Needs to move diagonally rightup
    when [2,1] then [1, 1]
    # Needs to move straight diagonally upleft
    when [-2,2] then [-1, 1]
    # Needs to move straight diagonally upright
    when [2,2] then [1, 1]
    # Needs to move straight diagonally downleft
    when [-2,-2] then [-1, -1]
    # Needs to move straight diagonally downright
    when [2,-2] then [1, -1]
    else
        [0,0]
    end
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