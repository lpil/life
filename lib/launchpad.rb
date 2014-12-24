#!/usr/bin/env ruby

require 'unimidi'

class Launchpad
  attr_accessor :input, :output

  def initialize
    @output = UniMIDI::Output.find { |e| e.name.match(/Launchpad/) }.open
    @input  = UniMIDI::Input.find  { |e| e.name.match(/Launchpad/) }.open
  end

  def light(x, y)
    @output.puts 144, grid_to_note(x, y), 1
  end

  def button_presses
    @input.gets.map do |msg|
      msg[:data]
         .each_slice(3)
         .to_a
         .reject { |e| e.last == 0 }
         .map { |e| note_to_grid e[1] }

    end.reject { |e| e == [] }
  end

  private

  def note_to_grid(note)
    y = note / 16
    x = note - y
    [x, y]
  end

  def grid_to_note(x, y)
    x + y * 16
  end
end

lp = Launchpad.new

loop do
  p lp.button_presses
  sleep 0.2
end
