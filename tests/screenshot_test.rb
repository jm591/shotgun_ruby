require 'rspec'
require 'tempfile'
require 'selenium-webdriver'
require_relative '../lib/shotgun_ruby'

RSpec.describe 'Screenshot Tests: ' do
    before do
        @driver = Selenium::WebDriver.for :firefox
        window = @driver.title
        @id = `xwininfo -name "Mozilla Firefox" | grep -Po '(?<=Window id: )[0-9a-zA-Z]+'`
        @imagefile = Tempfile.new(['testimage', '.pnm'])
    end

    it 'Screenshot should exist' do
        expect(File.size(@imagefile.path)).to be 0
        ShotgunRuby.screenshot(@id.strip, @imagefile.path)
        expect(File.size(@imagefile.path)).to be > 0
    end

    after do
        @driver.close
    end

end