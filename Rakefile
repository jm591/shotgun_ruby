# frozen_string_literal: true

require "rake/testtask"
require "rake/extensiontask"

SOURCE_PATTERN = "*.{rs,toml,lock,rb}"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

Rake::ExtensionTask.new("shotgun_ruby") do |ext|
  ext.lib_dir = "lib/shotgun_ruby"
  ext.source_pattern = SOURCE_PATTERN
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
end

task :build do
  require "bundler"

  spec = Bundler.load_gemspec("shotgun_ruby.gemspec")

  FileUtils.rm_rf("pkg/shotgun_ruby")

  spec.files.each do |file|
    FileUtils.mkdir_p("pkg/shotgun_ruby/#{File.dirname(file)}")
    FileUtils.cp(file, "pkg/shotgun_ruby/#{file}")
  end

  FileUtils.cp("shotgun_ruby.gemspec", "pkg/shotgun_ruby")

  full_path = File.expand_path("./../../../crates/rb-sys", __FILE__)
  cargo_toml_path = "pkg/shotgun_ruby/ext/shotgun_ruby/Cargo.toml"
  new_contents = File.read(cargo_toml_path).gsub("./../../../../crates/rb-sys", full_path)
  FileUtils.rm(cargo_toml_path)
  File.write(cargo_toml_path, new_contents)

  Dir.chdir("pkg/shotgun_ruby") do
    sh "gem build shotgun_ruby.gemspec --output=../shotgun_ruby.gem"
  end
end

task :install do
  Dir.chdir("pkg") do
    sh "gem install shotgun_ruby.gem"
  end
end

task default: %i[compile test install]
