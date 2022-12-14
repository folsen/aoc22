# Return a hashmap with stone coordinates
def read_input(input_file)
    grid = {}
    File.open(input_file).each do |line|
        edges = line.strip.split(" -> ")
        edges.map!{|e| e.split(",").map(&:to_i)}
        (0..edges.length-2).each do |i|
            if edges[i][0] <= edges[i+1][0]
                x_range = edges[i][0]..edges[i+1][0]
            else
                x_range = edges[i+1][0]..edges[i][0]
            end
            if edges[i][1] <= edges[i+1][1]
                y_range = edges[i][1]..edges[i+1][1]
            else
                y_range = edges[i+1][1]..edges[i][1]
            end
            x_range.each do |x|
                y_range.each do |y|
                    grid[[x,y]] = '#'
                end
            end
        end
    end
    grid
end

def add_pos(p1, p2)
    [p1[0] + p2[0], p1[1] + p2[1]]
end

def add_sand(part)
    grid = read_input("input.txt")
    abyss_level = grid.max_by{|k,v| k[1]}[0][1]
    floor_level = abyss_level+2
    max_reached = 0
    last_rest = nil
    while (part == 1 && max_reached <= abyss_level) || (part == 2 && last_rest != [500,0]) do
        sand = [500,0]
        at_rest = false
        while !at_rest && (part == 2 || max_reached <= abyss_level) do
            if part == 2 && sand[1] + 1 == floor_level
                at_rest = true
                grid[sand] = 'o'
                last_rest = sand
                break
            end
            if grid[add_pos(sand, [0,1])].nil?
                sand = add_pos(sand, [0,1])
            elsif grid[add_pos(sand, [-1,1])].nil?
                sand = add_pos(sand, [-1,1])
            elsif grid[add_pos(sand, [1,1])].nil?
                sand = add_pos(sand, [1,1])
            else
                at_rest = true
                grid[sand] = 'o'
                last_rest = sand
            end
            max_reached = sand[1]
        end
    end
    grid
end

grid = add_sand(1)
pp grid.filter{|k,v| v == 'o'}.length

grid = add_sand(2)
pp grid.filter{|k,v| v == 'o'}.length