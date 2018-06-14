SUBMODULES = testdata/Packages/.git

info:
	$(info Targets)
	$(info -----------------------------------------------------------------------)
	$(info assets      | generate default theme packs and syntax)
	$(info - OTHER TARGETS -------------------------------------------------------)
	$(info themes      | generate default theme pack)
	$(info packs       | generate default syntax pack)
	$(info syntest     | run syntax test summary)
	$(info pull-latest | update submodules and packs and produce syntax test summary)


$(SUBMODULES):
	git submodule update --init --recursive

assets: packs themes

packs: $(SUBMODULES)
	cargo run --example gendata -- synpack testdata/Packages assets/default_newlines.packdump assets/default_nonewlines.packdump

themes: $(SUBMODULES)
	cargo run --example gendata -- themepack testdata assets/default.themedump

syntest: $(SUBMODULES)
	@echo Tip: Run make update-known-failures to update the known failures file.
	cargo run --release --example syntest -- testdata/Packages testdata/Packages --summary | diff -U 1000000 testdata/known_syntest_failures.txt -
	@echo No new failures!

update-known-failures: $(SUBMODULES)
	cargo run --release --example syntest -- testdata/Packages testdata/Packages --summary | tee testdata/known_syntest_failures.txt

pull-latest: $(SUBMODULES)
	#git checkout master
	git fetch trishume
	git merge --ff-only trishume/master
	git submodule foreach git pull origin master
	$(MAKE) syntest
	$(MAKE) assets
