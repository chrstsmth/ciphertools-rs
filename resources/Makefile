src = ./src
target = ./target
cipher-target = ../target/release
cipher = $(cipher-target)/cipher
language-modeler = $(cipher-target)/language

all: clean generate-ciphertexts generate-language-models

.PHONY: generate-ciphertexts
generate-ciphertexts:
	ruby tools/generate-ciphertexts.rb $(src) $(target) $(cipher)

.PHONY: generate-language-models
generate-language-models:
	ruby tools/generate-language-models.rb $(src) $(target) $(language-modeler)

.PHONY: clean
clean:
	rm -rf target
