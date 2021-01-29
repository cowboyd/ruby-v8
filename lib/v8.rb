# frozen_string_literal: true

require_relative "v8/version"
require "rutie"

module V8
  class Error < StandardError; end

  Rutie.new(:rubyracer).init 'Init_rubyracer', __dir__
end
