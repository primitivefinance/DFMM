bind:
	@echo "Building project artifacts."
	forge bind --bindings-path kit/src/bindings --contracts src/ --skip-cargo-toml --module

tex:
	@echo "Building LaTeX report."
	xelatex -shell-escape -interaction nonstopmode -output-directory=report report/main.tex 

all: bind tex
