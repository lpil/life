#!/usr/bin/env ruby

# Our game
class Game
  def initialize(height, width)
    @board = Array.new(height) { Array.new(width) { Cell.new } }
  end

  def play
    loop do
      (system 'clear')
      puts self
      sleep 0.08
      next_gen
    end
  end

  def to_s
    @board.map do |row|
      row.map { |cell| cell.to_s }.join
    end.join "\n"
  end

  private

  def next_gen
    @board.each_with_index do |row, y|
      row.each_with_index do |cell, x|
        cell.neighbours = count_neighbours y, x
      end
    end
    @board.flatten.each { |cell| cell.next }
  end

  def count_neighbours(y, x)
    [
      [-1, -1], [-1,+0], [-1,+1],
      [+0, -1],          [+0,+1],
      [+1, -1], [+1,+0], [+1,+1]
    ].reduce(0) do |a, e|
      ny = (y + e[0]) % @board.size
      nx = (x + e[1]) % @board[0].size
      a + @board[ny][nx].to_i
    end
  end
end

# Cell
class Cell
  attr_writer :neighbours, :alive

  def initialize
    @alive = true if rand > 0.9
  end

  def next
    @alive = @alive ? (2..3).include?(@neighbours) : 3 == @neighbours
  end

  def to_i
    @alive ? 1 : 0
  end

  def to_s
    @alive ? 'o' : ' '
  end
end

begin
  Game.new(24, `tput cols`.to_i).play
rescue Interrupt => _
  puts
end
