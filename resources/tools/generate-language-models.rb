require 'fileutils'

src_path = ARGV[0]
target_path = ARGV[1]
language_modeler_cmd = ARGV[2]

wortschatz_path = "#{src_path}/wortschatz"
out_path = "#{target_path}/languages"
FileUtils.mkdir_p out_path

wortschatz = Dir["#{wortschatz_path}/**/*words.txt"]

wortschatz.each {|w|
	wortschatz_name = File.basename(w, File.extname(w)) + ".lang"

	language_path = "#{out_path}/#{wortschatz_name}"
	command = "#{language_modeler_cmd} #{w} > #{language_path}"

	puts command
	system(command)
}
