#!/usr/bin/env ruby

require_relative '../lib/game_of_life'
require_relative '../lib/launchpad'

game = GameOfLife.new 8, 8
lpad = Launchpad.new

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
    sleep 0.15
  end
end

loop do
  lpad.button_presses.each do |e|
    x = e[0]
    y = e[1]
    game.board[x][y].alive = true
  end
end
