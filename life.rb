#!/usr/bin/env ruby

class Game
  def initialize(height, width)
    @board = Array.new(height) { Array.new(width) { Cell.new } }
  end

  def to_s
    @board.map do |row|
      row.map { |cell| cell.to_s }.join
    end.join "\n"
  end
end

class Cell
  attr_writer :neighbours

  def next!
    @alive = @alive ? (2..3) === @neighbours : 3 == @neighbours
  end

  def to_i
    @alive ? 1 : 0
  end

  def to_s
    @alive ? '#' : ' '
  end
end
