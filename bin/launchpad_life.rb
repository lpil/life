#!/usr/bin/env ruby

require_relative '../lib/game_of_life'
require_relative '../lib/launchpad'

game = GameOfLife.new 8, 8
lpad = Launchpad.new
time = 0.05

Thread.new do
  loop do
    game.board.each.with_index do |col, x|
      col.each.with_index do |cell, y|
        if cell.alive
          lpad.light x, y
        else
          lpad.unlight x, y
        end
      end
    end
    game.next_gen!
    sleep time
  end
end

loop do
  lpad.button_presses.each do |e|
    x = e[0]
    y = e[1]
    # If inside the grid
    if x < 8
      game.board[x][y].alive = true
      lpad.light x, y, :red
    else
      time = 0.01 + y * 0.01
    end
  end
end
