### PART 1 ###
games = []
File.open("input.txt").each do |line|
    games.push(line.strip.split(' '))
end

def score_game(game)
    case game
    when ["A", "X"] # Rock vs Rock
        1 + 3
    when ["A", "Y"] # Rock vs Paper
        2 + 6
    when ["A", "Z"] # Rock vs Scissor
        3 + 0
    when ["B", "X"] # Paper vs Rock
        1 + 0
    when ["B", "Y"] # Paper vs Paper
        2 + 3
    when ["B", "Z"] # Paper vs Scissor
        3 + 6
    when ["C", "X"] # Scissor vs Rock
        1 + 6
    when ["C", "Y"] # Scissor vs Paper
        2 + 0
    when ["C", "Z"] # Scissor vs Scissor
        3 + 3
    end
end

total_score = 0

games.each do |game|
    total_score += score_game(game)
end

puts total_score

### PART 2 ###

def score_alternate(game)
    case game
    when ["A", "Z"] # Rock vs Paper (Win)
        2 + 6
    when ["A", "Y"] # Rock vs Rock (Draw)
        1 + 3
    when ["A", "X"] # Rock vs Scissor (Loss)
        3 + 0
    when ["B", "Z"] # Paper vs Scissor (Win)
        3 + 6               
    when ["B", "Y"] # Paper vs Paper (Draw)
        2 + 3               
    when ["B", "X"] # Paper vs Rock (Loss)
        1 + 0
    when ["C", "Z"] # Scissor vs Rock (Win)
        1 + 6                 
    when ["C", "Y"] # Scissor vs Scissor (Draw)
        3 + 3                 
    when ["C", "X"] # Scissor vs Paper (Loss)
        2 + 0
    end
end

total_score = 0

games.each do |game|
    total_score += score_alternate(game)
end

puts total_score