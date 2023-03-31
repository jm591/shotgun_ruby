# frozen_string_literal: true

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "shotgun_ruby/#{$1}/shotgun_ruby"
rescue LoadError
  require "shotgun_ruby/shotgun_ruby"
end

module ShotgunRuby
  class Error < StandardError; end
  # Your code goes here...
end
