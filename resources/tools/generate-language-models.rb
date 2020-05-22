require 'fileutils'

src_path = ARGV[0]
target_path = ARGV[1]
language_modeler_cmd = ARGV[2]

wortschatz_path = "#{src_path}/wortschatz"
language_path = "#{target_path}/languages"
out_path_windowed = "#{language_path}/windowed"
out_path_words = "#{language_path}/words"
FileUtils.mkdir_p out_path_windowed
FileUtils.mkdir_p out_path_words

wortschatz = Dir["#{wortschatz_path}/**/*words.txt"]

wortschatz.each {|w|
	wortschatz_name = File.basename(w, File.extname(w)) + ".lang"

	commands = []
	commands.push("#{language_modeler_cmd} #{w} > #{out_path_words}/#{wortschatz_name}")
	commands.push("#{language_modeler_cmd} --windowed #{w} > #{out_path_windowed}/#{wortschatz_name}")

	commands.each { |c|
		puts c
		system(c)
	}
}
