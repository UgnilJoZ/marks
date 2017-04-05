all:
	rustc -C opt-level=3 mark.rs
	ln -sf mark unmark
	ln -sf mark marked
clean:
	rm -f mark unmark marked
install: all
	cp mark unmark marked /usr/local/bin
