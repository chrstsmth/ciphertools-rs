require 'fileutils'
require 'pathname'

src_root = ARGV[0]
target_root = ARGV[1]
cipher_cmd = ARGV[2]

src_plaintexts_root = "#{src_root}/plaintexts"
src_keys_root = "#{src_root}/keys"

plaintexts = Dir["#{src_plaintexts_root}/**/*.txt"]
ciphers = Dir["#{src_keys_root}/*"]

ciphers.each {|c|
	if File.file?(c)
		cipher_name = File.basename(c, File.extname(c))
		keyfiles = [c]
	elsif File.directory?(c)
		cipher_name = File.basename(c)
		keyfiles = Dir["#{c}/**/*"]
	end

	keyfiles.each { |k|
		keys_path_component = Pathname.new(k).relative_path_from(Pathname.new(src_keys_root))
		keys_path_component = File.join(File.dirname(keys_path_component), File.basename(keys_path_component, ".*"))

		plaintexts.each { |p|
			plaintext_path_component = Pathname.new(p).relative_path_from(Pathname.new(src_plaintexts_root))
			plaintext_path_component = File.join(File.dirname(plaintext_path_component), File.basename(plaintext_path_component, ".*"))
			out_path = File.join(target_root, keys_path_component, plaintext_path_component)

			FileUtils.mkdir_p out_path

			keys = File.readlines(k, chomp: true)
			keys.each_with_index { |key, i|
				ciphertext_path = "#{out_path}/#{i}.txt"
				encipher_command = "#{cipher_cmd} #{cipher_name} encipher -p #{p} -k #{key} > #{ciphertext_path}"
				puts encipher_command
				system(encipher_command)
			}
		}
	}
}
