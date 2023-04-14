# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "shotgun_ruby"
  spec.version = "0.1.0"
  spec.authors = ["Jonas Moesicke"]
  spec.email = ["jmoesicke@clustermarket.com"]
  spec.homepage = "https://github.com/jm591/shotgun_ruby"

  spec.summary = "Rust Screenshot Tool"
  spec.description = "A ruby gem adaptation of the shotgun screenshot tool for linux. Creates a .pam screenshots at a given path and returns the location."
  spec.required_ruby_version = ">= 2.3.0"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/shotgun_ruby/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
