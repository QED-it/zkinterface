
pdf:
	latexmk -pdf -outdir=out zkinterop

live:
	latexmk -synctex=1 -interaction=nonstopmode -file-line-error -pdf -outdir=out zkinterop

clean:
	rm -r out

install:
	# pdfTeX, Version 3.14159265-2.6-1.40.18 (TeX Live 2017/Debian)
	sudo apt-get install texlive texlive-science texlive-latex-extra texlive-bibtex-extra latexmk
