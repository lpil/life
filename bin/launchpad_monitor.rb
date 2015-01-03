#!/usr/bin/env ruby

require_relative '../lib/launchpad'

lp = Launchpad.new
loop do
  p lp.button_presses
end
